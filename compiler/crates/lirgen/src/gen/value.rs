use sb_compiler_lirgen_ir::{lir, LirBlock, Add, Call, Li};
use sb_compiler_semcheck_hir::Value;

use super::{GenContext, FARG_REG_BASE, RET_REG, ZERO_REG, lirgen_expr};

pub fn lirgen_value<'src>(ctx: &mut GenContext<'src>, value: Value<'src>) -> LirBlock {
    let (result_reg, lirs) = match value {
        Value::Const { value, .. } => {
            let reg_imm = ctx.alloc_reg();
            (reg_imm, vec![lir!(Li(value) reg_imm)])
        }
        Value::Var { var, .. } => {
            (ctx.ref_var_reg(&var).unwrap(), vec![])
        }
        Value::Expr { expr, .. } => {
            let lir_expr = lirgen_expr(ctx, *expr);
            let reg_expr = lir_expr.result_reg();
            (reg_expr, vec![lir_expr])
        }
        Value::Call { name, args, .. } => {
            let mut lirs = vec![];
            for (idx, value) in args.into_iter().enumerate() {
                let lir_arg = lirgen_expr(ctx, value);
                let reg_arg = lir_arg.result_reg();
                lirs.push(lir_arg);
                lirs.push(lir!(Add FARG_REG_BASE + idx as u32, ZERO_REG, reg_arg));
            }
            lirs.push(lir!(Call(name)));
            (RET_REG, lirs)
        }
    };

    LirBlock::Single {
        result_reg,
        lirs,
    }
}
