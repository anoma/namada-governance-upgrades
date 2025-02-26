use namada_tx_prelude::*;

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    // 1. Enable NAM transfers
    let native_token_transferable_key = parameters_storage::get_native_token_transferable_key();
    ctx.write(&native_token_transferable_key, true)?;

    Ok(())
}
