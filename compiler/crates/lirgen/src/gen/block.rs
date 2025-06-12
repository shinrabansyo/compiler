use sb_compiler_parse_ast::Block;
use sb_compiler_lirgen_ir::LirBlock;

use crate::{GenContext, ZERO_REG};
use super::lirgen_stmt;

pub fn lirgen_block<'input>(ctx: &mut GenContext<'input>, block: &Block<'input>) -> LirBlock {
    let lirs = block.stmts
        .iter()
        .map(|ast| lirgen_stmt(ctx, ast))
        .collect();

    LirBlock::Single {
        result_reg: ZERO_REG,
        lirs,
    }
}
