use std::collections::BTreeSet;

use anyhow::{Context, Result};
use penumbra_proto::DomainType;
use penumbra_transaction::Transaction;

#[tracing::instrument(skip(tx))]
pub(super) fn valid_binding_signature(tx: &Transaction) -> Result<()> {
    let tx_body = tx.transaction_body().encode_to_vec();

    tracing::debug!(bvk = ?tx.binding_verification_key(), tx_body = ?tx_body);

    // Check binding signature.
    tx.binding_verification_key()
        .verify(&tx_body[..], tx.binding_sig())
        .context("binding signature failed to verify")
}

pub(super) fn no_duplicate_nullifiers(tx: &Transaction) -> Result<()> {
    // Disallow multiple `Spend`s with the same `Nullifier`.
    // This can't be implemented in the (`Spend`)[`crate::action_handler::actions::spend::Spend`] `ActionHandler`
    // because it requires access to the entire transaction, and we don't want to perform the check across the entire
    // transaction for *each* `Spend` within the transaction, only once.
    let mut spent_nullifiers = BTreeSet::new();
    for nf in tx.spent_nullifiers() {
        if let Some(duplicate) = spent_nullifiers.replace(nf) {
            return Err(anyhow::anyhow!(
                "Duplicate nullifier in transaction: {}",
                duplicate
            ));
        }
    }

    Ok(())
}
