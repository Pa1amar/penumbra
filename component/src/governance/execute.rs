use crate::{
    governance::proposal::Outcome,
    shielded_pool::{StateReadExt as _, StateWriteExt as _, SupplyWrite},
};

use super::{
    proposal::{self, chain_params},
    tally,
    view::StateWriteExt as _,
    StateReadExt as _,
};
use anyhow::{Context, Result};
use penumbra_chain::{StateReadExt as _, StateWriteExt};
use penumbra_crypto::ProposalNft;
use penumbra_storage::StateWrite;
use penumbra_transaction::action::{
    ProposalDepositClaim, ProposalPayload, ProposalSubmit, ProposalWithdraw, ValidatorVote,
    ValidatorVoteBody,
};
use tracing::instrument;

#[instrument(skip(state))]
pub async fn proposal_submit<S: StateWrite>(
    mut state: S,
    ProposalSubmit {
        proposal,
        deposit_amount,
    }: &ProposalSubmit,
) -> Result<()> {
    // Store the contents of the proposal and generate a fresh proposal id for it
    let proposal_id = state
        .new_proposal(proposal)
        .await
        .context("can create proposal")?;

    // Set the deposit amount for the proposal
    state.put_deposit_amount(proposal_id, *deposit_amount).await;

    // Register the denom for the voting proposal NFT
    state
        .register_denom(&ProposalNft::voting(proposal_id).denom())
        .await?;

    // Set the proposal state to voting (votes start immediately)
    state
        .put_proposal_state(proposal_id, proposal::State::Voting)
        .await
        .context("can set proposal state")?;

    // Determine what block it is currently, and calculate when the proposal should start voting
    // (now!) and finish voting (later...), then write that into the state
    let chain_params = state
        .get_chain_params()
        .await
        .context("can get chain params")?;
    let current_block = state
        .get_block_height()
        .await
        .context("can get block height")?;
    let voting_end = current_block + chain_params.proposal_voting_blocks;
    state
        .put_proposal_voting_start(proposal_id, current_block)
        .await;
    state.put_proposal_voting_end(proposal_id, voting_end).await;

    // If there was a proposal submitted, ensure we track this so that clients
    // can retain state needed to vote as delegators
    let mut compact_block = state.stub_compact_block();
    compact_block.proposal_started = true;
    state.stub_put_compact_block(compact_block);

    tracing::debug!(proposal = %proposal_id, "created proposal");

    Ok(())
}

#[instrument(skip(state))]
pub async fn proposal_withdraw<S: StateWrite>(
    mut state: S,
    ProposalWithdraw { proposal, reason }: &ProposalWithdraw,
) -> Result<()> {
    state
        .put_proposal_state(
            *proposal,
            proposal::State::Withdrawn {
                reason: reason.clone(),
            },
        )
        .await
        .context("proposal withdraw succeeds")?;

    // Register the denom for the withdrawn proposal NFT
    state
        .register_denom(&ProposalNft::withdrawn(*proposal).denom())
        .await?;

    tracing::debug!(proposal = %proposal, "withdrew proposal");

    Ok(())
}

#[instrument(skip(state))]
pub async fn validator_vote<S: StateWrite>(
    mut state: S,
    ValidatorVote {
        auth_sig: _,
        body:
            ValidatorVoteBody {
                proposal,
                vote,
                identity_key,
                governance_key: _, // This is only used for checks so that stateless verification can be done on the signature
            },
    }: &ValidatorVote,
) -> Result<()> {
    state
        .cast_validator_vote(*proposal, *identity_key, *vote)
        .await;

    tracing::debug!(proposal = %proposal, "cast validator vote");

    Ok(())
}

#[instrument(skip(state))]
pub async fn proposal_deposit_claim<S: StateWrite>(
    mut state: S,
    ProposalDepositClaim {
        proposal,
        deposit_amount: _, // not needed to transition state; deposit is self-minted in tx
        outcome: resupplied_outcome,
    }: &ProposalDepositClaim,
) -> Result<()> {
    // The only effect of doing a deposit claim is to state transition the proposal to claimed so it
    // cannot be claimed again. The deposit amount is self-minted in the transaction (proof of
    // deserving-ness is the supplied proposal NFT, which is burned in the transaction), so we don't
    // need to distribute it here.

    if let Some(proposal::State::Finished { outcome }) = state.proposal_state(*proposal).await? {
        // This should be prevented by earlier checks, but replicating here JUST IN CASE!
        if *resupplied_outcome != outcome.as_ref().map(|_| ()) {
            anyhow::bail!(
                "proposal {} has outcome {:?}, but deposit claim has outcome {:?}",
                proposal,
                outcome,
                resupplied_outcome
            );
        }

        // Register the denom for the claimed proposal NFT
        state
            .register_denom(
                &match &outcome {
                    Outcome::Passed => ProposalNft::passed(*proposal),
                    Outcome::Failed { .. } => ProposalNft::failed(*proposal),
                    Outcome::Vetoed { .. } => ProposalNft::vetoed(*proposal),
                }
                .denom(),
            )
            .await?;

        // Set the proposal state to claimed
        state
            .put_proposal_state(*proposal, proposal::State::Claimed { outcome })
            .await?;
    } else {
        anyhow::bail!("proposal {} is not in finished state", proposal);
    }
    Ok(())
}

// TODO: fill in when delegator votes happen
// pub async fn delegator_vote(state: &State, delegator_vote: &DelegatorVote) {}

#[instrument(skip(state))]
pub async fn enact_all_passed_proposals<S: StateWrite>(mut state: S) -> Result<()> {
    let parameters = tally::Parameters::new(&state)
        .await
        .context("can generate tally parameters")?;

    let circumstance = tally::Circumstance::new(&state)
        .await
        .context("can generate tally circumstance")?;

    // For every unfinished proposal, conclude those that finish in this block
    for proposal_id in state
        .unfinished_proposals()
        .await
        .context("can get unfinished proposals")?
    {
        // TODO: tally delegator votes
        if let Some(outcome) = parameters
            .tally(&state, circumstance, proposal_id)
            .await
            .context("can tally proposal")?
        {
            tracing::debug!(proposal = %proposal_id, outcome = ?outcome, "proposal voting finished");

            // If the proposal passes, enact it now
            if outcome.is_passed() {
                enact_proposal(&mut state, proposal_id).await?;
            }

            // Log the result
            tracing::info!(proposal = %proposal_id, outcome = match outcome {
                Outcome::Passed => "passed",
                Outcome::Failed { .. } => "failed",
                Outcome::Vetoed {.. } => "vetoed",
            }, "voting concluded");

            // Record the outcome of the proposal: this is especially important for emergency
            // proposals, because it prevents the vote from continuing after they are passed
            state
                .put_proposal_state(proposal_id, proposal::State::Finished { outcome })
                .await
                .context("can put finished proposal outcome")?;
        }
    }

    Ok(())
}

#[instrument(skip(state))]
async fn enact_proposal<S: StateWrite>(mut state: S, proposal_id: u64) -> Result<()> {
    let payload = state
        .proposal_payload(proposal_id)
        .await
        .context("can get proposal payload")?
        .context("proposal payload is present")?;

    match payload {
        ProposalPayload::Signaling { .. } => {
            // Nothing to do for signaling proposals
        }
        ProposalPayload::Emergency { halt_chain } => {
            let height = state
                .get_block_height()
                .await
                .context("can get block height")?;

            // If the proposal calls to halt the chain...
            if halt_chain {
                // Check to see if the operator has set the environment variable indicating they
                // wish to resume from this particular chain halt, i.e. the chain has already halted
                // and they are bringing it back up again
                if std::env::var("PD_RESUME_FROM_EMERGENCY_HALT_PROPOSAL")
                    .ok()
                    .and_then(|v| v.parse::<u64>().ok()) // value of var must be number
                    .filter(|&resume_from| resume_from == proposal_id) // number must be this proposal's id (to prevent an always-on resume functionality)
                    .is_some()
                {
                    // If so, just print an information message, and don't halt the chain
                    tracing::info!(proposal = %proposal_id, %height, "resuming from emergency chain halt");
                } else {
                    // If not, print an informational message and immediately exit the process
                    tracing::error!(proposal = %proposal_id, %height, "emergency proposal passed, calling for immediate chain halt");
                    std::process::exit(0);
                }
            }
        }
        ProposalPayload::ParameterChange {
            effective_height: _,
            new_parameters,
        } => {
            let height = state
                .get_block_height()
                .await
                .context("can get block height")?;

            // Since other proposals may have changed the chain parameters in the meantime,
            // and parameter validation must ensure consistency across all parameters, we
            // need to perform a final validation step prior to applying the new parameters.
            let old_parameters = state
                .get_chain_params()
                .await
                .context("can get chain parameters")?;

            if !chain_params::is_valid_stateless(&new_parameters)
                || !chain_params::is_valid_stateful(&new_parameters, &old_parameters)
            {
                // The parameters are invalid, so we cannot apply them.
                tracing::info!(proposal = %proposal_id, %height, "chain param proposal passed, however the new parameters are invalid");
                // TODO: should there be a more descriptive error message here?
                return Err(anyhow::anyhow!("invalid chain parameters, could not apply"));
            }

            // Apply the new (valid) parameter changes immediately:
            let new_params = chain_params::resolve_parameters(&new_parameters, &old_parameters)
                .context("can resolve validated parameters")?;

            state.put_chain_params(new_params);
        }
        ProposalPayload::DaoSpend {
            schedule_transactions: _,
            cancel_transactions: _,
        } => {
            // TODO: schedule transaction cancellations by removing the first matching one from the
            // front of the schedule for their effective block
            // TODO: schedule new transactions by appending them to the end of the schedule for their
            // effective block
            // TODO: don't forget to fill in the part in the shielded pool where the transactions
            // actually get included in a block
            todo!("implement daospend execution")
        }
    }

    Ok(())
}

pub async fn enact_pending_parameter_changes<S: StateWrite>(_state: S) -> Result<()> {
    // TODO: read the new parameters for this block, if any, and change the chain params to reflect
    // them. Parameters should be stored in the state as a map from name to value string.
    Ok(())
}
