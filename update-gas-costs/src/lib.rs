use std::collections::BTreeMap;

use namada_tx_prelude::*;

pub type ChannelId = &'static str;
pub type BaseToken = &'static str;

pub type Gas = token::Amount;
pub type MinimumGasPrice = Gas;

const IBC_TOKENS: [(ChannelId, BaseToken, MinimumGasPrice); 6] = [
    (
        "channel-0",
        "stuosmo",
        Gas::from_u64(3), // 3 stuosmo / gas unit
    ),
    (
        "channel-1",
        "uosmo",
        Gas::from_u64(3), // 3 uosmo / gas unit
    ),
    (
        "channel-4",
        "upenumbra",
        Gas::from_u64(3), // 3 upenumbra / gas unit;
    ),
    (
        "channel-5",
        "uusdc",
        Gas::from_u64(1), // 1 uusdc / gas unit;
    ),
    (
        "channel-6",
        "unym",
        Gas::from_u64(15), // 15 unym / gas unit;
    ),
    (
        "channel-7",
        "untrn",
        Gas::from_u64(7), // 7 untrn / gas unit;
    ),
];

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    // Read the current gas cost map
    let gas_cost_key = get_gas_cost_key();
    let mut minimum_gas_price: BTreeMap<Address, token::Amount> =
        ctx.read(&gas_cost_key)?.unwrap_or_default();

    // Enable IBC deposit/withdraws limits
    for (channel_id, base_token, can_be_used_as_gas) in IBC_TOKENS {
        let ibc_denom = format!("transfer/{channel_id}/{base_token}");
        let token_address = ibc::ibc_token(&ibc_denom).clone();

        // Check if this ibc token should can also be used to pay for gas
        minimum_gas_price.insert(token_address.clone(), gas);
    }

    // Write the gas cost map back to storage
    ctx.write(&gas_cost_key, minimum_gas_price)?;

    Ok(())
}
