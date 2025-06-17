use masp::MaspEpoch;
use masp::{encode_asset_type, Precision};
use masp_primitives::convert::AllowedConversion;
use masp_primitives::transaction::components::I128Sum;
use namada_tx_prelude::*;
use std::collections::BTreeMap;
use std::str::FromStr;
use token::storage_key::{
    masp_conversion_key, masp_scheduled_base_native_precision_key,
    masp_scheduled_reward_precision_key,
};
use token::{Denomination, MaspDigitPos};

pub type ChannelId = &'static str;
pub type BaseToken = &'static str;

/// Represents a Namada address in Bech32m encoding
pub type AddressBech32m = &'static str;

/// A convenience data structure to allow token addresses to be more readably
/// expressed as a channel ID and base token instead of a raw Namada address.
pub enum TokenAddress {
    // Specify an IBC address. This can also be done more directly using the
    // Self::Address variant.
    Ibc(ChannelId, BaseToken),
    // Directly specify a Namada address
    Address(AddressBech32m),
}

// The address of the native token. This is what rewards are denominated in.
const NATIVE_TOKEN_BECH32M: AddressBech32m = "tnam1q9gr66cvu4hrzm0sd5kmlnjje82gs3xlfg3v6nu7";
// The tokens whose rewarrds will be reset.
const TOKENS: [(TokenAddress, Denomination, Precision); 7] = [
    (
        TokenAddress::Ibc("channel-7", "uosmo"),
        Denomination(0u8),
        10_000_000,
    ),
    (
        TokenAddress::Ibc("channel-8", "stuatom"),
        Denomination(0u8),
        80_000,
    ),
    (
        TokenAddress::Ibc("channel-8", "stutia"),
        Denomination(0u8),
        80_000,
    ),
    (
        TokenAddress::Ibc("channel-8", "stuosmo"),
        Denomination(0u8),
        1_000_000_000,
    ),
    (
        TokenAddress::Ibc("channel-9", "uatom"),
        Denomination(0u8),
        80_000,
    ),
    (
        TokenAddress::Ibc("channel-13", "utia"),
        Denomination(0u8),
        5_000_000,
    ),
    (
        TokenAddress::Address(NATIVE_TOKEN_BECH32M),
        Denomination(6u8),
        1_000_000_000,
    ),
];

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    // The address of the native token. This is what rewards are denominated in.
    let native_token =
        Address::from_str(NATIVE_TOKEN_BECH32M).expect("unable to construct native token address");
    // The MASP epoch in which this migration will be applied. This number
    // controls the number of epochs of conversions created.
    let target_masp_epoch: MaspEpoch =
        MaspEpoch::try_from_epoch(Epoch(4248), 4).expect("failed to construct target masp epoch");

    // Reset the allowed conversions for the above tokens
    for (token_address, denomination, precision) in TOKENS {
        // Compute the Namada address
        let token_address = match token_address {
            TokenAddress::Ibc(channel_id, base_token) => {
                let ibc_denom = format!("transfer/{channel_id}/{base_token}");
                ibc::ibc_token(&ibc_denom).clone()
            }
            TokenAddress::Address(addr) => {
                Address::from_str(addr).expect("unable to construct token address")
            }
        };

        // Erase the TOK rewards that have been distributed so far
        let mut asset_types = BTreeMap::new();
        let mut precision_toks = BTreeMap::new();
        let mut reward_deltas = BTreeMap::new();
        // TOK[ep, digit]
        let mut asset_type = |epoch, digit| {
            *asset_types.entry((epoch, digit)).or_insert_with(|| {
                encode_asset_type(token_address.clone(), denomination, digit, Some(epoch))
                    .expect("unable to encode asset type")
            })
        };
        // PRECISION TOK[ep, digit]
        let mut precision_tok = |epoch, digit| {
            precision_toks
                .entry((epoch, digit))
                .or_insert_with(|| {
                    AllowedConversion::from(I128Sum::from_pair(
                        asset_type(epoch, digit),
                        i128::try_from(precision).expect("precision too large"),
                    ))
                })
                .clone()
        };
        // -PRECISION TOK[ep, digit] + PRECISION TOK[ep+1, digit]
        let mut reward_delta = |epoch, digit| {
            reward_deltas
                .entry((epoch, digit))
                .or_insert_with(|| {
                    -precision_tok(epoch, digit) + precision_tok(epoch.next().unwrap(), digit)
                })
                .clone()
        };
        // The key holding the shielded reward precision of current token
        let shielded_token_reward_precision_key =
            masp_scheduled_reward_precision_key(&target_masp_epoch, &token_address);

        ctx.write(&shielded_token_reward_precision_key, precision)?;
        // If the current token is the native token, then also update the base
        // native precision
        if token_address == native_token {
            let shielded_token_base_native_precision_key =
                masp_scheduled_base_native_precision_key(&target_masp_epoch);

            ctx.write(&shielded_token_base_native_precision_key, precision)?;
        }
        // Write the new TOK conversions to memory
        for digit in MaspDigitPos::iter() {
            // -PRECISION TOK[ep, digit] + PRECISION TOK[current_ep, digit]
            let mut reward: AllowedConversion = I128Sum::zero().into();
            for epoch in MaspEpoch::iter_bounds_inclusive(
                MaspEpoch::zero(),
                target_masp_epoch.prev().unwrap(),
            )
            .rev()
            {
                // TOK[ep, digit]
                let asset_type =
                    encode_asset_type(token_address.clone(), denomination, digit, Some(epoch))
                        .expect("unable to encode asset type");
                reward += reward_delta(epoch, digit);
                // Write the conversion update to memory
                ctx.write(
                    &masp_conversion_key(&target_masp_epoch, &asset_type),
                    reward.clone(),
                )?;
            }
        }
    }

    Ok(())
}
