use sb_compiler_parse_ast::VarDecl;
use sb_compiler_lirgen_ir::LirBlock;

use crate::GenContext;
use super::lirgen_expr;

pub fn lirgen_var_decl(ctx: &mut GenContext, var_decl: &VarDecl) -> LirBlock {
    let lir_expr = lirgen_expr(ctx, &var_decl.expr);
    let reg_expr = lir_expr.result_reg();

    ctx.set_var_reg(var_decl.ident.clone(), reg_expr);

    LirBlock::Single {
        result_reg: reg_expr,
        lirs: vec![lir_expr],
    }
}
