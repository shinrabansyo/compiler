use sb_compiler_lirgen_ir::LirBlock;
use sb_compiler_semcheck_hir::VarDecl;

use crate::GenContext;
use super::lirgen_expr;

pub fn lirgen_var_decl<'src>(ctx: &mut GenContext<'src>, var_decl: &VarDecl<'src>) -> LirBlock {
    let lir_expr = lirgen_expr(ctx, &var_decl.expr);
    let reg_expr = lir_expr.result_reg();

    ctx.set_var_reg(var_decl.var, reg_expr);

    LirBlock::Single {
        result_reg: reg_expr,
        lirs: vec![lir_expr],
    }
}
