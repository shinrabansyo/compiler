use sb_compiler_lirgen_ir::{lir, LirBlock, Add, Call, Li};
use sb_compiler_semcheck_hir::Value;

use crate::{GenContext, ZERO_REG, RET_REG, FARG_REG_BASE};
use super::lirgen_expr;

pub fn lirgen_value(ctx: &mut GenContext, value: &Value) -> LirBlock {
    let (result_reg, lirs) = match value {
        Value::Const { value, .. } => {
            let reg_imm = ctx.alloc_reg();
            (reg_imm, vec![lir!(Li(*value) reg_imm)])
        }
        Value::Var { id, .. } => {
            (ctx.ref_var_reg(id).unwrap(), vec![])
        }
        Value::Expr { expr, .. } => {
            let lir_expr = lirgen_expr(ctx, expr);
            let reg_expr = lir_expr.result_reg();
            (reg_expr, vec![lir_expr])
        }
        Value::Call { call, .. } => {
            let mut lirs = vec![];
            for (idx, value) in call.args.iter().enumerate() {
                let lir_arg = lirgen_value(ctx, value);
                let reg_arg = lir_arg.result_reg();
                lirs.push(lir_arg);
                lirs.push(lir!(Add FARG_REG_BASE + idx as u32, ZERO_REG, reg_arg));
            }
            lirs.push(lir!(Call(format!("global.{}", call.ident.as_str()))));
            (RET_REG, lirs)
        }
    };

    LirBlock::Single {
        result_reg,
        lirs,
    }
}
