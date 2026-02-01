use sb_compiler_lirgen_ir::{lir, Add, LirBlock};
use sb_compiler_semcheck_hir::VarDecl;

use crate::GenContext;
use super::{ZERO_REG, RET_REG, lirgen_expr};

pub fn lirgen_var_decl<'src>(ctx: &mut GenContext<'src>, var_decl: VarDecl<'src>) -> LirBlock {
    let lir_expr = lirgen_expr(ctx, var_decl.expr);
    let reg_expr = lir_expr.result_reg();

    let lir_var = if reg_expr == RET_REG {
        let reg_var = ctx.alloc_reg();
        LirBlock::Single {
            result_reg: reg_var,
            lirs: vec![
                lir!(Add reg_var, ZERO_REG, RET_REG),
            ],
        }
    } else {
        LirBlock::Single { result_reg: reg_expr, lirs: vec![] }
    };
    ctx.set_var_reg(var_decl.var, lir_var.result_reg());

    LirBlock::Single {
        result_reg: reg_expr,
        lirs: vec![
            lir_expr,
            lir_var,
        ],
    }
}
