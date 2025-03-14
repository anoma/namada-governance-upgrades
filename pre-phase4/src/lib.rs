use namada_tx_prelude::*;
use masp_primitives::transaction::components::I128Sum;
use std::collections::BTreeMap;
use masp::encode_asset_type;
use masp_primitives::convert::AllowedConversion;
use masp::MaspEpoch;
use token::storage_key::{masp_conversion_key, masp_reward_precision_key};
use token::{Denomination, MaspDigitPos};

pub type ChannelId = &'static str;
pub type BaseToken = &'static str;
// Valid precisions must be in the intersection of i128 and u128
pub type Precision = i128;

// The denomination of the targetted token. Since all tokens here are IBC
// tokens, this is 0.
const DENOMINATION: Denomination = Denomination(0u8);

const IBC_TOKENS: [(ChannelId, BaseToken, Precision); 6] = [
    ("channel-1", "uosmo", 1000i128),
    ("channel-2", "uatom", 1000i128),
    ("channel-3", "utia", 1000i128),
    ("channel-0", "stuosmo", 1000i128),
    ("channel-0", "stuatom", 1000i128),
    ("channel-0", "stutia", 1000i128),
];

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    // The MASP epoch in which this migration will be applied. This number
    // controls the number of epochs of conversions created.
    let target_masp_epoch: MaspEpoch = MaspEpoch::try_from_epoch(Epoch(8000), 4)
        .expect("failed to construct target masp epoch");
    
    // Enable IBC deposit/withdraws limits
    for (channel_id, base_token, precision) in IBC_TOKENS {
        let ibc_denom = format!("transfer/{channel_id}/{base_token}");
        let token_address = ibc::ibc_token(&ibc_denom).clone();

        // Write some null MASP reward data
        let shielded_token_reward_precision_key =
            masp_reward_precision_key(&token_address);

        ctx.write(&shielded_token_reward_precision_key, precision)?;

        // Erase the TOK rewards that have been distributed so far
        let mut asset_types = BTreeMap::new();
        let mut precision_toks = BTreeMap::new();
        let mut reward_deltas = BTreeMap::new();
        // TOK[ep, digit]
        let mut asset_type = |epoch, digit| {
            *asset_types.entry((epoch, digit)).or_insert_with(|| {
                encode_asset_type(
                    token_address.clone(),
                    DENOMINATION,
                    digit,
                    Some(epoch),
                )
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
                        precision,
                    ))
                })
                .clone()
        };
        // -PRECISION TOK[ep, digit] + PRECISION TOK[ep+1, digit]
        let mut reward_delta = |epoch, digit| {
            reward_deltas
                .entry((epoch, digit))
                .or_insert_with(|| {
                    -precision_tok(epoch, digit)
                        + precision_tok(epoch.next().unwrap(), digit)
                })
                .clone()
        };
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
                let asset_type = encode_asset_type(
                    token_address.clone(),
                    DENOMINATION,
                    digit,
                    Some(epoch),
                )
                .expect("unable to encode asset type");
                reward += reward_delta(epoch, digit);
                // Write the conversion update to memory
                ctx.write(&masp_conversion_key(&asset_type), reward.clone())?;
            }
        }
    }

    Ok(())
}
