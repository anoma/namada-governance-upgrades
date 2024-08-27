use std::str::FromStr;

use dec::Dec;
use namada_tx_prelude::*;

pub const NAM_MAX_REWARD: &str = "0.01";
pub const NAM_TARGET_LOCKED_AMOUNT: u64 = 1_000_000_000;
pub const KP_GAIN: &str = "120000";
pub const KD_GAIN: &str = "120000";

pub const MIN_PROPOSAL_GRACE_EPOCHS: u64 = 8;
pub const MIN_PROPOSAL_VOTING_PERIOD: u64 = 28;

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    let nam_address = ctx.get_native_token()?;

    // 1. Enable NAM transfers
    let native_token_transferable_key = parameters_storage::get_native_token_transferable_key();
    ctx.write(&native_token_transferable_key, true)?;

    // 2. Enable MASP rewards for NAM
    let shielded_token_last_inflation_key =
        token::storage_key::masp_last_inflation_key(&nam_address);
    let shielded_token_last_locked_amount_key =
        token::storage_key::masp_last_locked_amount_key(&nam_address);
    let shielded_token_max_rewards_key = token::storage_key::masp_max_reward_rate_key(&nam_address);
    let shielded_token_target_locked_amount_key =
        token::storage_key::masp_locked_amount_target_key(&nam_address);
    let shielded_token_kp_gain_key = token::storage_key::masp_kp_gain_key(&nam_address);
    let shielded_token_kd_gain_key = token::storage_key::masp_kd_gain_key(&nam_address);
    let token_map_key = token::storage_key::masp_token_map_key();

    // Add native token to the masp token map
    let mut token_map: masp::TokenMap = ctx.read(&token_map_key)?.unwrap_or_default();
    token_map.insert("nam".to_string(), nam_address);
    ctx.write(&token_map_key, token_map)?;

    // Write the MASP inflation keys
    ctx.write(&shielded_token_last_inflation_key, token::Amount::zero())?;
    ctx.write(
        &shielded_token_last_locked_amount_key,
        token::Amount::zero(),
    )?;
    ctx.write(
        &shielded_token_max_rewards_key,
        Dec::from_str(NAM_MAX_REWARD).unwrap(),
    )?;
    ctx.write(
        &shielded_token_target_locked_amount_key,
        token::Amount::from_uint(NAM_TARGET_LOCKED_AMOUNT, 6_u8).unwrap(),
    )?;
    ctx.write(&shielded_token_kp_gain_key, Dec::from_str(KP_GAIN).unwrap())?;
    ctx.write(&shielded_token_kd_gain_key, Dec::from_str(KD_GAIN).unwrap())?;

    // 3. Update governance parameters
    let min_proposal_grace_epochs_key = gov_storage::keys::get_min_proposal_grace_epochs_key();
    ctx.write(&min_proposal_grace_epochs_key, MIN_PROPOSAL_GRACE_EPOCHS)?;

    let min_proposal_voting_period_key = gov_storage::keys::get_min_proposal_voting_period_key();
    ctx.write(&min_proposal_voting_period_key, MIN_PROPOSAL_VOTING_PERIOD)?;

    Ok(())
}
