use namada_tx_prelude::*;

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    let max_tx_bytes_key = parameters_storage::get_max_tx_bytes_key();

    ctx.write(&max_tx_bytes_key, 2_000_000_u64)?;

    Ok(())
}
