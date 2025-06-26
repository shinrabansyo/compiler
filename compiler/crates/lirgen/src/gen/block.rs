use sb_compiler_lirgen_ir::LirBlock;
use sb_compiler_semcheck_hir::Block;

use super::{GenContext, ZERO_REG, lirgen_stmt};

pub fn lirgen_block(ctx: &mut GenContext, block: Block) -> LirBlock {
    let lirs = block.stmts
        .into_iter()
        .map(|ast| lirgen_stmt(ctx, ast))
        .collect();

    LirBlock::Single {
        result_reg: ZERO_REG,
        lirs,
    }
}
