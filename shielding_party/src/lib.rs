use namada_tx_prelude::*;

pub type ChannelId = &'static str;
pub type BaseToken = &'static str;

pub type MintTokenLimit = token::Amount;
pub type ThroughtputTokenLimit = token::Amount;

const IBC_TOKENS: [(ChannelId, BaseToken, MintTokenLimit, ThroughtputTokenLimit); 1] = [(
    "channel-0",
    "uosmo",
    MintTokenLimit::from_u64(10000000000),
    ThroughtputTokenLimit::from_u64(10000000000),
)];

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    // Enable IBC deposit/withdraws limits
    for (channel_id, base_token, mint_limit, throughput_limit) in IBC_TOKENS {
        let ibc_denom = format!("transfer/{channel_id}/{base_token}");
        let token_address = ibc::ibc_token(&ibc_denom);

        let mint_limit_token_key = ibc::mint_limit_key(&token_address);
        ctx.write(&mint_limit_token_key, mint_limit)?;

        let throughput_limit_token_key = ibc::throughput_limit_key(&token_address);
        ctx.write(&throughput_limit_token_key, throughput_limit)?;
    }

    Ok(())
}
