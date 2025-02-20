use namada_ibc::parameters::{IbcParameters, IbcTokenRateLimits};
use namada_ibc::storage::params_key;
use namada_tx_prelude::*;

pub type ChannelId = &'static str;
pub type BaseToken = &'static str;

pub type MintTokenLimit = token::Amount;
pub type ThroughtputTokenLimit = token::Amount;
pub type CanBeUsedAsGas = bool;
pub type Gas = token::Amount;
pub type MinimumGasPrice = Option<Gas>;

const MINT_LIMIT: MintTokenLimit = MintTokenLimit::from_u64(100000000);
const THROUGHPUT_LIMIT: ThroughtputTokenLimit = ThroughtputTokenLimit::from_u64(100000000);

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    let ibc_params_key = params_key();

    let ibc_params = IbcParameters {
        default_rate_limits: IbcTokenRateLimits {
            mint_limit: MINT_LIMIT,
            throughput_per_epoch_limit: THROUGHPUT_LIMIT,
        },
    };
    ctx.write(&ibc_params_key, ibc_params)?;

    Ok(())
}
