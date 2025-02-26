use namada_tx_prelude::*;

pub type ChannelId = &'static str;
pub type BaseToken = &'static str;

pub type MintTokenLimit = token::Amount;
pub type ThroughtputTokenLimit = token::Amount;

const MINT_LIMIT: MintTokenLimit = MintTokenLimit::from_u128(100_000_000_000_000); // Ex: 100M NAM -> Need to update with final number
const THROUGHPUT_LIMIT: ThroughtputTokenLimit =
    ThroughtputTokenLimit::from_u128(10_000_000_000_000); // Ex: 10M NAM -> Need to update with final number

pub const MIN_PROPOSAL_GRACE_EPOCHS: u64 = 8;
pub const MIN_PROPOSAL_VOTING_PERIOD: u64 = 28;

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    // 1. Enable IBC rate limits for native token
    let native_token = ctx.get_native_token()?;
    let mint_limit_token_key = ibc::mint_limit_key(&native_token);
    let throughput_limit_token_key = ibc::throughput_limit_key(&native_token);

    ctx.write(&mint_limit_token_key, MINT_LIMIT)?;
    ctx.write(&throughput_limit_token_key, THROUGHPUT_LIMIT)?;

    // 2. Update governance parameters
    let min_proposal_grace_epochs_key = gov_storage::keys::get_min_proposal_grace_epochs_key();
    let min_proposal_voting_period_key = gov_storage::keys::get_min_proposal_voting_period_key();

    ctx.write(&min_proposal_grace_epochs_key, MIN_PROPOSAL_GRACE_EPOCHS)?;
    ctx.write(&min_proposal_voting_period_key, MIN_PROPOSAL_VOTING_PERIOD)?;

    Ok(())
}
