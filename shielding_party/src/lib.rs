use namada_tx_prelude::*;

#[transaction]
fn apply_tx(_ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    log_string("asd12");
    Ok(())
}
