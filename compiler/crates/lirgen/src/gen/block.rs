use sb_compiler_parse_ast::Block;
use sb_compiler_lirgen_ir::LirTree;

use crate::{GenContext, ZERO_REG};
use super::lirgen_stmt;

pub fn lirgen_block(ctx: &mut GenContext, block: &Block) -> LirTree {
    let lirs = block.stmts
        .iter()
        .map(|ast| lirgen_stmt(ctx, ast))
        .collect();

    LirTree::Single {
        result_reg: ZERO_REG,
        lirs,
    }
}
