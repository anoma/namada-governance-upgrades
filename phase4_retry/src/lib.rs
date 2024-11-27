use namada_tx_prelude::*;

pub type Denomination = u8;
pub type ChannelId = &'static str;
pub type BaseToken = &'static str;

pub type TokenTargetLockedAmount = u64;

const IBC_TOKENS: [(Denomination, ChannelId, BaseToken, TokenTargetLockedAmount); 1] = [(
    0,
    "channel-1",
    "uosmo",
    1_000_000_000, // Target shielded amount of 1,000 OSMO (1B uOSMO)
)];

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    // Enable shielded set rewards for ibc tokens
    for (denomination, channel_id, base_token, target_locked_amount) in IBC_TOKENS {
        let ibc_denom = format!("transfer/{channel_id}/{base_token}");
        let token_address = ibc::ibc_token(&ibc_denom);

        let shielded_token_target_locked_amount_key =
            token::storage_key::masp_locked_amount_target_key(&token_address);

        ctx.write(
            &shielded_token_target_locked_amount_key,
            token::Amount::from_uint(target_locked_amount, denomination).unwrap(),
        )?;
    }

    Ok(())
}
