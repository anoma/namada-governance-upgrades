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

const IBC_TOKENS: [(
    Denomination,
    ChannelId,
    BaseToken,
    TokenMaxReward,
    TokenTargetLockedAmount,
    KpGain,
    KdGain,
); 6] = [
    (
        0,
        "channel-7",
        "uosmo",
        "0.02",     // Max 2% annual inflation
        60_000_000, // Target 60 OSMO
        "120000",
        "120000",
    ),
    (
        0,
        "channel-9",
        "uatom",
        "0.02",     // Max 2% annual inflation
        60_000_000, // Target 60 ATOM
        "120000",
        "120000",
    ),
    (
        0,
        "channel-10",
        "utia",
        "0.02",     // Max 2% annual inflation
        60_000_000, // Target 60 TIA
        "120000",
        "120000",
    ),
    (
        0,
        "channel-8",
        "stuosmo",
        "0.02",     // Max 2% annual inflation
        60_000_000, // Target 60 stOSMO
        "120000",
        "120000",
    ),
    (
        0,
        "channel-8",
        "stuatom",
        "0.02",     // Max 2% annual inflation
        60_000_000, // Target 60 stATOM
        "120000",
        "120000",
    ),
    (
        0,
        "channel-8",
        "stutia",
        "0.02",     // Max 2% annual inflation
        60_000_000, // Target 60 stTIA
        "120000",
        "120000",
    ),
];

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
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

        // Read the current balance of the IBC token in MASP and set that as initial locked amount
        let ibc_balance_key = balance_key(
            &token_address,
            &Address::Internal(address::InternalAddress::Masp),
        );
        let current_ibc_amount = ctx.read::<token::Amount>(&ibc_balance_key)?.unwrap();
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

    Ok(())
}
