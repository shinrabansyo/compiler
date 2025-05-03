use sb_compiler_parse_ast::Expr;
use sb_compiler_lirgen_ir::LirTree;

use crate::GenContext;
use super::lirgen_assign;

pub fn lirgen_expr(ctx: &mut GenContext, expr: &Expr) -> LirTree {
    lirgen_assign(ctx, &expr.assign)
}
