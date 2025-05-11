use sb_compiler_parse_ast::VarDecl;
use sb_compiler_lirgen_ir::LirTree;

use crate::GenContext;
use super::lirgen_expr;

pub fn lirgen_var_decl(ctx: &mut GenContext, var_decl: &VarDecl) -> LirTree {
    let lir_expr = lirgen_expr(ctx, &var_decl.expr);
    let reg_expr = lir_expr.result_reg();

    ctx.sym_table.insert(var_decl.ident.clone(), reg_expr);

    LirTree::Single {
        result_reg: reg_expr,
        lirs: vec![lir_expr],
    }
}
