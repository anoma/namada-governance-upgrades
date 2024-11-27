use namada_tx_prelude::*;

const NAM_MINT_LIMIT: u64 = 1_000_000;
const NAM_THROUGHPUT_LIMIT: u64 = 1_000_000;

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    let token_address = ctx.get_native_token()?;

    let mint_limit_token_key = ibc::mint_limit_key(&token_address);
    ctx.write(
        &mint_limit_token_key,
        token::Amount::native_whole(NAM_MINT_LIMIT),
    )?;

    let throughput_limit_token_key = ibc::throughput_limit_key(&token_address);
    ctx.write(
        &throughput_limit_token_key,
        token::Amount::native_whole(NAM_THROUGHPUT_LIMIT),
    )?;

    Ok(())
}
