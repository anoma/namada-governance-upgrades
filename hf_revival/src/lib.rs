use std::{collections::BTreeMap, str::FromStr};

use dec::Dec;
use namada_tx_prelude::*;
use parameters_storage::get_gas_cost_key;

pub type ChannelId = &'static str;
pub type BaseToken = &'static str;
pub type Denomination = u8;

pub type MintTokenLimit = token::Amount;
pub type ThroughtputTokenLimit = token::Amount;
pub type CanBeUsedAsGas = bool;
pub type Gas = token::Amount;
pub type MinimumGasPrice = Option<Gas>;

pub type TokenMaxReward = &'static str;
pub type TokenTargetLockedAmount = u64;
pub type KpGain = &'static str;
pub type KdGain = &'static str;

#[allow(clippy::type_complexity)]
const IBC_TOKENS: [(
    Denomination,
    ChannelId,
    BaseToken,
    MintTokenLimit,
    ThroughtputTokenLimit,
    MinimumGasPrice,
    TokenMaxReward,
    TokenTargetLockedAmount,
    KpGain,
    KdGain,
); 6] = [
    (
        0,
        "channel-15",
        "stuosmo",
        MintTokenLimit::from_u64(100_000_000),
        ThroughtputTokenLimit::from_u64(100_000_000),
        Some(Gas::from_u64(1)),
        "0.02",
        60_000_000,
        "120000",
        "120000",
    ),
    (
        0,
        "channel-15",
        "stuatom",
        MintTokenLimit::from_u64(100_000_000),
        ThroughtputTokenLimit::from_u64(100_000_000),
        Some(Gas::from_u64(1)),
        "0.02",
        60_000_000,
        "120000",
        "120000",
    ),
    (
        0,
        "channel-15",
        "stutia",
        MintTokenLimit::from_u64(100_000_000),
        ThroughtputTokenLimit::from_u64(100_000_000),
        Some(Gas::from_u64(1)),
        "0.02",
        60_000_000,
        "120000",
        "120000",
    ),
    (
        0,
        "channel-16",
        "uatom",
        MintTokenLimit::from_u64(100_000_000),
        ThroughtputTokenLimit::from_u64(100_000_000),
        Some(Gas::from_u64(1)),
        "0.02",
        60_000_000,
        "120000",
        "120000",
    ),
    (
        0,
        "channel-17",
        "utia",
        MintTokenLimit::from_u64(100_000_000),
        ThroughtputTokenLimit::from_u64(100_000_000),
        Some(Gas::from_u64(1)),
        "0.02",
        60_000_000,
        "120000",
        "120000",
    ),
    (
        0,
        "channel-18",
        "uosmo",
        MintTokenLimit::from_u64(100_000_000),
        ThroughtputTokenLimit::from_u64(100_000_000),
        Some(Gas::from_u64(1)),
        "0.02",
        60_000_000,
        "120000",
        "120000",
    ),
];

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    // Read the current gas cost map
    let gas_cost_key = get_gas_cost_key();
    let mut minimum_gas_price: BTreeMap<Address, token::Amount> =
        ctx.read(&gas_cost_key)?.unwrap_or_default();

    // Read the current MASP token map
    let token_map_key = token::storage_key::masp_token_map_key();
    let mut token_map = ctx
        .read::<masp::TokenMap>(&token_map_key)?
        .unwrap_or_default();

    // Enable IBC deposit/withdraws limits
    for (
        denomination,
        channel_id,
        base_token,
        mint_limit,
        throughput_limit,
        can_be_used_as_gas,
        token_max_reward,
        token_target_locked_amount,
        kp_gain,
        kd_gain,
    ) in IBC_TOKENS
    {
        let ibc_denom = format!("transfer/{channel_id}/{base_token}");
        let token_address = ibc::ibc_token(&ibc_denom).clone();

        let mint_limit_token_key = ibc::mint_limit_key(&token_address);
        ctx.write(&mint_limit_token_key, mint_limit)?;

        let throughput_limit_token_key = ibc::throughput_limit_key(&token_address);
        ctx.write(&throughput_limit_token_key, throughput_limit)?;

        // Check if this ibc token should can also be used to pay for gas
        if let Some(gas) = can_be_used_as_gas {
            minimum_gas_price.insert(token_address.clone(), gas);
        }

        // Add the ibc token to the masp token map
        token_map.insert(ibc_denom, token_address.clone());

        // Write some null MASP reward data
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

        ctx.write(
            &shielded_token_last_locked_amount_key,
            token::Amount::zero(),
        )?;
        ctx.write(&shielded_token_last_inflation_key, token::Amount::zero())?;
        ctx.write(
            &shielded_token_max_rewards_key,
            Dec::from_str(token_max_reward).unwrap(),
        )?;
        ctx.write(
            &shielded_token_target_locked_amount_key,
            token::Amount::from_uint(token_target_locked_amount, denomination).unwrap(),
        )?;
        ctx.write(&shielded_token_kp_gain_key, Dec::from_str(kp_gain).unwrap())?;
        ctx.write(&shielded_token_kd_gain_key, Dec::from_str(kd_gain).unwrap())?;
    }

    // Write the gas cost map back to storage
    ctx.write(&gas_cost_key, minimum_gas_price)?;

    // Write the token map back to storage
    ctx.write(&token_map_key, token_map)?;

    Ok(())
}
