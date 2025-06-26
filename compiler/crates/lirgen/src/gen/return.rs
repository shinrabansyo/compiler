use sb_compiler_lirgen_ir::{lir, LirBlock, Add, FnReturn};
use sb_compiler_semcheck_hir::Return;

use super::{GenContext, RET_REG, ZERO_REG, lirgen_expr};

pub fn lirgen_return(ctx: &mut GenContext, r#return: &Return) -> LirBlock {
    let lir_expr = lirgen_expr(ctx, &r#return.expr);
    let reg_expr = lir_expr.result_reg();

    LirBlock::Single {
        result_reg: ZERO_REG,
        lirs: vec![
            lir_expr,
            lir!(Add RET_REG, ZERO_REG, reg_expr),
            lir!(FnReturn),
        ],
    }
}
