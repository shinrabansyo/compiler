use sb_compiler_lirgen_ir::{lir, LirBlock, Add, Sub, ShiftL, ShiftR, ShiftRa};
use sb_compiler_semcheck_hir::Assign;

use super::{GenContext, ZERO_REG, lirgen_logic_or};

pub fn lirgen_assign(ctx: &mut GenContext, assign: &Assign) -> LirBlock {
    let (result_reg, lirs) = match assign {
        Assign::Normal { var, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, assign);
            let reg_assign = lir_assign.result_reg();

            let reg_var = ctx.ref_var_reg(&var.symbol).unwrap();

            (reg_var, vec![lir_assign, lir!(Add reg_var, ZERO_REG, reg_assign)])
        }
        Assign::Plus { var, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, assign);
            let reg_assign = lir_assign.result_reg();

            let reg_var = ctx.ref_var_reg(&var.symbol).unwrap();

            (reg_var, vec![lir_assign, lir!(Add reg_var, reg_var, reg_assign)])
        }
        Assign::Minus { var, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, assign);
            let reg_assign = lir_assign.result_reg();

            let reg_var = ctx.ref_var_reg(&var.symbol).unwrap();

            (reg_var, vec![lir_assign, lir!(Sub reg_var, reg_var, reg_assign)])
        }
        Assign::ShiftL { var, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, assign);
            let reg_result = lir_assign.result_reg();

            let reg_var = ctx.ref_var_reg(&var.symbol).unwrap();

            (reg_var, vec![lir_assign, lir!(ShiftL reg_var, reg_var, reg_result)])
        }
        Assign::ShiftR { var, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, assign);
            let reg_result = lir_assign.result_reg();

            let reg_var = ctx.ref_var_reg(&var.symbol).unwrap();

            (reg_var, vec![lir_assign, lir!(ShiftR reg_var, reg_var, reg_result)])
        }
        Assign::ShiftRa { var, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, assign);
            let reg_result = lir_assign.result_reg();

            let reg_var = ctx.ref_var_reg(&var.symbol).unwrap();

            (reg_var, vec![lir_assign, lir!(ShiftRa reg_var, reg_var, reg_result)])
        }
        Assign::LogicOr { or, .. } => {
            return lirgen_logic_or(ctx, or);
        }
    };

    LirBlock::Single {
        result_reg,
        lirs,
    }
}
