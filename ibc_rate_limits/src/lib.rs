use namada_ibc::storage::{mint_limit_key, throughput_limit_key};
use namada_tx_prelude::*;
use namada_tx_prelude::token::Amount;

const BASE_DENOM: &str = "transfer/08-wasm-1369/0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2";

pub type ChannelId = &'static str;
pub type BaseDenom = &'static str;

pub type MintLimit = Amount;
pub type ThroughputLimit = Amount;

const IBC_TOKENS: [(
    ChannelId,
    BaseDenom,
    MintLimit,
    ThroughputLimit,
); 1] = [
    (
        "channel-9",
        BASE_DENOM,
        Amount::from_u64(10000),
        Amount::from_u64(10000),
    ),
];

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {

    for (channel_id, base_token, mint_limit, throughput_limit) in
        IBC_TOKENS
    {
        let ibc_denom = format!("transfer/{channel_id}/{base_token}");
        let token_address = ibc::ibc_token(&ibc_denom);
        let key_mint_limit = mint_limit_key(&token_address);
        let key_throughput_limit = throughput_limit_key(&token_address);
        ctx.write(&key_mint_limit, mint_limit)?;
        ctx.write(&key_throughput_limit, throughput_limit)?;
    }


    Ok(())
}
