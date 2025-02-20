use namada_tx_prelude::*;

pub type ChannelId = &'static str;
pub type BaseToken = &'static str;

pub type MintTokenLimit = token::Amount;
pub type ThroughtputTokenLimit = token::Amount;
pub type CanBeUsedAsGas = bool;
pub type Gas = token::Amount;
pub type MinimumGasPrice = Option<Gas>;

const MINT_LIMIT: MintTokenLimit = MintTokenLimit::from_u64(10000000000);
const THROUGHPUT_LIMIT: ThroughtputTokenLimit = ThroughtputTokenLimit::from_u64(10000000000);

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {

    // Enable IBC deposit/withdraws limits for NAM native token
    let token_address = ctx.get_native_token()?;

    let mint_limit_token_key = ibc::mint_limit_key(&token_address);
    ctx.write(&mint_limit_token_key, MINT_LIMIT)?;

    let throughput_limit_token_key = ibc::throughput_limit_key(&token_address);
    ctx.write(&throughput_limit_token_key, THROUGHPUT_LIMIT)?;

    Ok(())
}
