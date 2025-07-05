use sb_compiler_lirgen_ir::LirBlock;
use sb_compiler_semcheck_hir::Expr;

use crate::GenContext;
use super::lirgen_assign;

pub fn lirgen_expr<'src>(ctx: &mut GenContext<'src>, expr: Expr<'src>) -> LirBlock {
    lirgen_assign(ctx, expr.assign)
}
