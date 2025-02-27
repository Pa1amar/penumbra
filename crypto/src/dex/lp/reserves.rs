use crate::asset::Amount;
use penumbra_proto::{
    client::v1alpha1::StubCpmmReservesResponse, core::dex::v1alpha1 as pb, DomainType,
};

use super::position::MAX_RESERVE_AMOUNT;

/// The reserves of a position.
///
/// Like a position, this implicitly treats the trading function as being
/// between assets 1 and 2, without specifying what those assets are, to avoid
/// duplicating data (each asset ID alone is four times the size of the
/// reserves).
#[derive(Debug, Clone)]
pub struct Reserves {
    pub r1: Amount,
    pub r2: Amount,
}

impl Reserves {
    pub fn check_bounds(&self) -> anyhow::Result<()> {
        if self.r1.value() as u128 > MAX_RESERVE_AMOUNT
            || self.r2.value() as u128 > MAX_RESERVE_AMOUNT
        {
            Err(anyhow::anyhow!(format!(
                "Reserve amounts are out-of-bounds (limit: {MAX_RESERVE_AMOUNT})"
            )))
        } else {
            Ok(())
        }
    }
}

impl DomainType for Reserves {
    type Proto = pb::Reserves;
}

impl TryFrom<pb::Reserves> for Reserves {
    type Error = anyhow::Error;

    fn try_from(value: pb::Reserves) -> Result<Self, Self::Error> {
        Ok(Self {
            r1: value
                .r1
                .ok_or_else(|| anyhow::anyhow!("missing r1"))?
                .try_into()?,
            r2: value
                .r2
                .ok_or_else(|| anyhow::anyhow!("missing r2"))?
                .try_into()?,
        })
    }
}

impl From<Reserves> for pb::Reserves {
    fn from(value: Reserves) -> Self {
        Self {
            r1: Some(value.r1.into()),
            r2: Some(value.r2.into()),
        }
    }
}

impl TryFrom<StubCpmmReservesResponse> for Reserves {
    type Error = anyhow::Error;

    fn try_from(value: StubCpmmReservesResponse) -> Result<Self, Self::Error> {
        value
            .reserves
            .ok_or_else(|| anyhow::anyhow!("empty StubCpmmReservesResponse message"))?
            .try_into()
    }
}
