use sb_compiler_lirgen_ir::LirBlock;
use sb_compiler_semcheck_hir::Expr;

use crate::GenContext;
use super::lirgen_assign;

pub fn lirgen_expr(ctx: &mut GenContext, expr: &Expr) -> LirBlock {
    lirgen_assign(ctx, &expr.assign)
}
