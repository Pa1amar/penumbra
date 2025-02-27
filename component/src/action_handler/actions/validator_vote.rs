use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use penumbra_storage::{StateRead, StateWrite};
use penumbra_transaction::{action::ValidatorVote, Transaction};
use tracing::instrument;

use crate::{
    action_handler::ActionHandler,
    governance::{check, execute},
};

#[async_trait]
impl ActionHandler for ValidatorVote {
    #[instrument(name = "validator_vote", skip(self, _context))]
    async fn check_stateless(&self, _context: Arc<Transaction>) -> Result<()> {
        check::stateless::validator_vote(self)
    }

    #[instrument(name = "validator_vote", skip(self, state))]
    async fn check_stateful<S: StateRead>(&self, state: Arc<S>) -> Result<()> {
        check::stateful::validator_vote(&state, self).await
    }

    #[instrument(name = "validator_vote", skip(self, state))]
    async fn execute<S: StateWrite>(&self, state: S) -> Result<()> {
        execute::validator_vote(state, self).await
    }
}
