use namada_tx_prelude::{
    gov_storage::keys::get_min_proposal_voting_period_key,
    parameters::ProposalBytes,
    parameters_storage::{get_max_block_gas_key, get_max_proposal_bytes_key},
    *,
};

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    let max_proposal_bytes_key = get_max_proposal_bytes_key();
    let min_voting_period_key = get_min_proposal_voting_period_key();
    let max_block_gas_key = get_max_block_gas_key();

    ctx.write(
        &max_proposal_bytes_key,
        ProposalBytes::new(2_000_000).unwrap(),
    )?;
    ctx.write(&min_voting_period_key, 2_u64)?;
    ctx.write(&max_block_gas_key, 10_000_000_u64)?;

    Ok(())
}
