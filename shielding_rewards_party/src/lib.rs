use dec::Dec;
use namada_tx_prelude::*;
use std::str::FromStr;

pub type Denomination = u8;
pub type ChannelId = &'static str;
pub type BaseToken = &'static str;

pub type TokenMaxReward = &'static str;
pub type TokenTargetLockedAmount = u64;
pub type KpGain = &'static str;
pub type KdGain = &'static str;

const IBC_TOKENS: [(
    Denomination,
    ChannelId,
    BaseToken,
    TokenMaxReward,
    TokenTargetLockedAmount,
    KpGain,
    KdGain,
); 2] = [
    (
        0,
        "channel-0",
        "tnam1qrdm8ymq2svrrafzuqahm547xm4kfuw3aue93uzs",
        "0.01",
        1_000_000_000,
        "120000",
        "120000",
    ),
    (
        0,
        "channel-1",
        "tnam1qqx4luqsngxdmpf5nk8shkn7wwlmz6g7dckp8kgm",
        "0.015",
        1_500_000_000,
        "150000",
        "110000",
    ),
];

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    // Read the current MASP token map
    let token_map_key = token::storage_key::masp_token_map_key();
    let mut token_map = ctx
        .read::<masp::TokenMap>(&token_map_key)?
        .unwrap_or_default();

    let nam_address = ctx.get_native_token()?;

    // Add native token to token map
    token_map.insert("nam".to_string(), nam_address.clone());

    let shielded_native_token_last_inflation_key =
        token::storage_key::masp_last_inflation_key(&nam_address);
    let shielded_native_token_last_locked_amount_key =
        token::storage_key::masp_last_locked_amount_key(&nam_address);
    let shielded_native_token_max_rewards_key =
        token::storage_key::masp_max_reward_rate_key(&nam_address);
    let shielded_native_token_target_locked_amount_key =
        token::storage_key::masp_locked_amount_target_key(&nam_address);
    let shielded_native_token_kp_gain_key = token::storage_key::masp_kp_gain_key(&nam_address);
    let shielded_native_token_kd_gain_key = token::storage_key::masp_kd_gain_key(&nam_address);

    // Setup native token shielded set rewards to 0
    ctx.write(
        &shielded_native_token_last_inflation_key,
        token::Amount::zero(),
    )?;
    ctx.write(
        &shielded_native_token_last_locked_amount_key,
        token::Amount::zero(),
    )?;
    ctx.write(
        &shielded_native_token_max_rewards_key,
        Dec::zero(),
    )?;
    ctx.write(
        &shielded_native_token_target_locked_amount_key,
        token::Amount::from_uint(0, 6).unwrap(),
    )?;
    ctx.write(
        &shielded_native_token_kp_gain_key,
        Dec::zero(),
    )?;
    ctx.write(
        &shielded_native_token_kd_gain_key,
        Dec::zero(),
    )?;

    // Enable shielded set rewards for ibc tokens
    for (denomination, channel_id, base_token, max_reward, target_locked_amount, kp, kd) in
        IBC_TOKENS
    {
        let ibc_denom = format!("transfer/{channel_id}/{base_token}");
        let token_address = ibc::ibc_token(&ibc_denom);

        let shielded_token_last_inflation_key =
            token::storage_key::masp_last_inflation_key(&token_address);
        let shielded_token_last_locked_amount_key =
            token::storage_key::masp_last_locked_amount_key(&token_address);
        let shielded_token_max_rewards_key =
            token::storage_key::masp_max_reward_rate_key(&token_address);
        let shielded_token_target_locked_amount_key =
            token::storage_key::masp_locked_amount_target_key(&token_address);
        let shielded_token_kp_gain_key = token::storage_key::masp_kp_gain_key(&token_address);
        let shielded_token_kd_gain_key = token::storage_key::masp_kd_gain_key(&token_address);

        // Add the ibc token to the masp token map
        token_map.insert(ibc_denom, token_address);

        // Write the MASP inflation keys
        ctx.write(&shielded_token_last_inflation_key, token::Amount::zero())?;
        ctx.write(
            &shielded_token_last_locked_amount_key,
            token::Amount::zero(),
        )?;
        ctx.write(
            &shielded_token_max_rewards_key,
            Dec::from_str(max_reward).unwrap(),
        )?;
        ctx.write(
            &shielded_token_target_locked_amount_key,
            token::Amount::from_uint(target_locked_amount, denomination).unwrap(),
        )?;
        ctx.write(&shielded_token_kp_gain_key, Dec::from_str(kp).unwrap())?;
        ctx.write(&shielded_token_kd_gain_key, Dec::from_str(kd).unwrap())?;
    }

    ctx.write(&token_map_key, token_map)?;

    Ok(())
}
