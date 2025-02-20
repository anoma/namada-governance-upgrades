use dec::Dec;
use namada_tx_prelude::*;

use std::str::FromStr;

use namada_proof_of_stake::storage::{read_pos_params, write_pos_params};

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    let mut pos_params = read_pos_params::<Ctx, governance::Store<Ctx>>(ctx)?.owned;
    pos_params.target_staked_ratio = Dec::from_str("1").unwrap();
    write_pos_params(ctx, &pos_params)?;

    Ok(())
}
