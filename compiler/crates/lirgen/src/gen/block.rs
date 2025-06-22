use sb_compiler_lirgen_ir::LirBlock;
use sb_compiler_semcheck_hir::Block;

use crate::{GenContext, ZERO_REG};
use super::lirgen_stmt;

pub fn lirgen_block(ctx: &mut GenContext, block: &Block) -> LirBlock {
    let lirs = block.stmts
        .iter()
        .map(|ast| lirgen_stmt(ctx, ast))
        .collect();

    LirBlock::Single {
        result_reg: ZERO_REG,
        lirs,
    }
}
