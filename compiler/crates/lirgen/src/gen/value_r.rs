use sb_compiler_lirgen_ir::{lir, LirBlock, Add, Call, Li};
use sb_compiler_semcheck_hir::ValueR;

use super::{
    GenContext, FARG_REG_BASE, RET_REG, ZERO_REG,
    lirgen_expr, lirgen_struct_init, lirgen_struct_access,
};

pub fn lirgen_value_r<'src>(ctx: &mut GenContext<'src>, value: ValueR<'src>) -> LirBlock {
    let (result_reg, lirs) = match value {
        ValueR::Const { value, .. } => {
            let reg_imm = ctx.alloc_reg();
            (reg_imm, vec![lir!(Li(value) reg_imm)])
        }
        ValueR::Var { var, .. } => {
            (ctx.ref_var_reg(&var).unwrap(), vec![])
        }
        ValueR::Expr { expr, .. } => {
            let lir_expr = lirgen_expr(ctx, *expr);
            let reg_expr = lir_expr.result_reg();
            (reg_expr, vec![lir_expr])
        }
        ValueR::Call { name, args, .. } => {
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
        ValueR::StructInit { struct_init, .. } => {
            return lirgen_struct_init(ctx, struct_init);
        }
        ValueR::StructAccess { struct_access } => {
            return lirgen_struct_access(ctx, struct_access);
        }
    };

    LirBlock::Single {
        result_reg,
        lirs,
    }
}
