use dec::Dec;
use namada_tx_prelude::*;
use std::str::FromStr;
use token::storage_key::balance_key;

pub type Denomination = u8;
pub type ChannelId = &'static str;
pub type BaseToken = &'static str;

pub type TokenMaxReward = &'static str;
pub type TokenTargetLockedAmount = u64;
pub type KpGain = &'static str;
pub type KdGain = &'static str;

pub type MintTokenLimit = token::Amount;
pub type ThroughtputTokenLimit = token::Amount;

// Allow both OSMO and ATOM into Namada
const IBC_TOKENS_1: [(ChannelId, BaseToken, MintTokenLimit, ThroughtputTokenLimit); 2] = [
    (
        "channel-2",
        "uosmo",
        MintTokenLimit::from_u64(10000000000),
        ThroughtputTokenLimit::from_u64(10000000000),
    ),
    (
        "channel-1",
        "uatom",
        MintTokenLimit::from_u64(10000000000),
        ThroughtputTokenLimit::from_u64(10000000000),
    ),
];

// Only incentivize OSMO but add both to token map
const IBC_TOKENS_2: [(
    Denomination,
    ChannelId,
    BaseToken,
    TokenMaxReward,
    TokenTargetLockedAmount,
    KpGain,
    KdGain,
); 2] = [
    (0, "channel-1", "uatom", "0", 0, "0", "0"),
    (
        0,
        "channel-2",
        "uosmo",
        "0.01",
        1_000_000,
        "120000",
        "120000",
    ),
];

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    // Read the current MASP token map
    let token_map_key = token::storage_key::masp_token_map_key();
    let mut token_map = ctx
        .read::<masp::TokenMap>(&token_map_key)?
        .unwrap_or_default();

    // Enable IBC deposit/withdraws limits
    for (channel_id, base_token, mint_limit, throughput_limit) in IBC_TOKENS_1 {
        let ibc_denom = format!("transfer/{channel_id}/{base_token}");
        let token_address = ibc::ibc_token(&ibc_denom);

        let mint_limit_token_key = ibc::mint_limit_key(&token_address);
        ctx.write(&mint_limit_token_key, mint_limit)?;

        let throughput_limit_token_key = ibc::throughput_limit_key(&token_address);
        ctx.write(&throughput_limit_token_key, throughput_limit)?;
    }

    // Enable shielded set rewards for ibc tokens
    for (denomination, channel_id, base_token, max_reward, target_locked_amount, kp, kd) in
        IBC_TOKENS_2
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
        token_map.insert(ibc_denom, token_address.clone());

        // Read the current balance of the IBC token in MASP and set that as initial locked amount
        let ibc_balance_key = balance_key(
            &token_address,
            &Address::Internal(address::InternalAddress::Masp),
        );
        let current_ibc_amount = ctx
            .read::<token::Amount>(&ibc_balance_key)?
            .unwrap_or_default();
        ctx.write(&shielded_token_last_locked_amount_key, current_ibc_amount)?;

        // Initialize the remaining MASP inflation keys
        ctx.write(&shielded_token_last_inflation_key, token::Amount::zero())?;

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
