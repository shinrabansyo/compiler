use sb_compiler_parse_ast::VarDecl;
use sb_compiler_lirgen_ir::{lir, LirTree, Add};

use crate::{GenContext, ZERO_REG};
use super::lirgen_expr;

pub fn lirgen_var_decl(ctx: &mut GenContext, var_decl: &VarDecl) -> LirTree {
    let reserved_reg_range_start = ctx.reserved_regs;
    let reserved_label_range_start = ctx.reserved_labels;

    let lir_expr = lirgen_expr(ctx, &var_decl.expr);
    let reg_expr = lir_expr.reserved_reg_range().1 - 1;

    let reg_var = ctx.alloc_reg();
    ctx.sym_table.insert(var_decl.ident.clone(), reg_var);

    let reserved_reg_range = (reserved_reg_range_start, ctx.reserved_regs);
    let reserved_label_range = (reserved_label_range_start, ctx.reserved_labels);

    LirTree::Node {
        reserved_reg_range,
        reserved_label_range,
        lirs: vec![
            lir_expr,
            lir!(Add reg_var, ZERO_REG, reg_expr),
        ],
    }
}
