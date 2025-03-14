use namada_tx_prelude::*;

pub type ChannelId = &'static str;
pub type BaseToken = &'static str;
pub type Precision = u128;

const IBC_TOKENS: [(ChannelId, BaseToken, Precision); 6] = [
    ("channel-1", "uosmo", 1000u128),
    ("channel-2", "uatom", 1000u128),
    ("channel-3", "utia", 1000u128),
    ("channel-0", "stuosmo", 1000u128),
    ("channel-0", "stuatom", 1000u128),
    ("channel-0", "stutia", 1000u128),
];

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    // Enable IBC deposit/withdraws limits
    for (channel_id, base_token, precision) in IBC_TOKENS {
        let ibc_denom = format!("transfer/{channel_id}/{base_token}");
        let token_address = ibc::ibc_token(&ibc_denom).clone();

        // Write some null MASP reward data
        let shielded_token_reward_precision_key =
            token::storage_key::masp_reward_precision_key(&token_address);

        ctx.write(&shielded_token_reward_precision_key, precision)?;
    }

    Ok(())
}
