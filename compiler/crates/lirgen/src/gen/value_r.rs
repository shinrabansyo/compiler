use sb_compiler_lirgen_ir::{lir, LirBlock, Li, Lb, Lh, Lw};
use sb_compiler_semcheck_hir::ValueR;

use super::{GenContext, lirgen_call, lirgen_expr, lirgen_struct_init, lirgen_struct_access};

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
        ValueR::Call { call } => {
            return lirgen_call(ctx, call);
        }
        ValueR::StructInit { struct_init, .. } => {
            return lirgen_struct_init(ctx, struct_init);
        }
        ValueR::StructAccess { struct_access } => {
            let size = struct_access.ty.size();

            let lir_addr = lirgen_struct_access(ctx, struct_access);
            let reg_addr = lir_addr.result_reg();

            let reg_read = ctx.alloc_reg();
            let lir_read = match size {
                1 => lir!(Lb(0) reg_read, reg_addr),
                2 => lir!(Lh(0) reg_read, reg_addr),
                4 => lir!(Lw(0) reg_read, reg_addr),
                _ => unreachable!(),
            };

            (reg_read, vec![lir_addr, lir_read])
        }
    };

    LirBlock::Single {
        result_reg,
        lirs,
    }
}
