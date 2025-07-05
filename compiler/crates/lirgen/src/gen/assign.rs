use sb_compiler_lirgen_ir::{lir, LirBlock, Add, Bne, Blt, Jmp, JmpLabel, Sub, Li,ShiftL, ShiftR, ShiftRa};
use sb_compiler_semcheck_hir::Assign;

use super::{GenContext, ZERO_REG, lirgen_logic_or};

pub fn lirgen_assign<'src>(ctx: &mut GenContext<'src>, assign: Assign<'src>) -> LirBlock {
    let (result_reg, lirs) = match assign {
        Assign::Normal { var, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, *assign);
            let reg_assign = lir_assign.result_reg();

            let reg_var = ctx.ref_var_reg(&var).unwrap();

            (reg_var, vec![lir_assign, lir!(Add reg_var, ZERO_REG, reg_assign)])
        }
        Assign::Plus { var, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, *assign);
            let reg_assign = lir_assign.result_reg();

            let reg_var = ctx.ref_var_reg(&var).unwrap();

            (reg_var, vec![lir_assign, lir!(Add reg_var, reg_var, reg_assign)])
        }
        Assign::Minus { var, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, *assign);
            let reg_assign = lir_assign.result_reg();

            let reg_var = ctx.ref_var_reg(&var).unwrap();

            (reg_var, vec![lir_assign, lir!(Sub reg_var, reg_var, reg_assign)])
        }
        Assign::Mul { var, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, *assign);
            let reg_assign = lir_assign.result_reg();

            let reg_var = ctx.ref_var_reg(&var).unwrap();

            let reg_one = ctx.alloc_reg();
            let reg_cnt = ctx.alloc_reg();
            let reg_result = ctx.alloc_reg();

            let label_cond = ctx.alloc_label();
            let label_end = ctx.alloc_label();

            (
                reg_var,
                vec![
                    lir_assign,
                    lir!(Add reg_cnt, ZERO_REG, reg_assign),
                    lir!(Li(1) reg_one),
                    lir!(Li(0) reg_result),
                    lir!(Label label_cond),
                    lir!(Bne(12) ZERO_REG, reg_cnt, ZERO_REG),
                    lir!(JmpLabel(label_end)),
                    lir!(Add reg_result, reg_result, reg_var),
                    lir!(Sub reg_cnt, reg_cnt, reg_one),
                    lir!(JmpLabel(label_cond)),
                    lir!(Label label_end),
                    lir!(Add reg_var, ZERO_REG, reg_result),
                ],
            )
        }
        Assign::Div { var, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, *assign);
            let reg_assign = lir_assign.result_reg();

            let reg_var = ctx.ref_var_reg(&var).unwrap();

            let reg_one = ctx.alloc_reg();
            let reg_result = ctx.alloc_reg();

            let label_cond = ctx.alloc_label();
            let label_end = ctx.alloc_label();

            (
                reg_var,
                vec![
                    lir_assign,
                    lir!(Li(1) reg_one),
                    lir!(Li(0) reg_result),
                    lir!(Label label_cond),
                    lir!(Bne(12) ZERO_REG, reg_assign, ZERO_REG),
                    lir!(JmpLabel(label_end)),
                    lir!(Blt(12) ZERO_REG, reg_var, reg_assign),
                    lir!(Jmp(12)),
                    lir!(JmpLabel(label_end)),
                    lir!(Add reg_result, reg_result, reg_one),
                    lir!(Sub reg_var, reg_var, reg_assign),
                    lir!(JmpLabel(label_cond)),
                    lir!(Label label_end),
                    lir!(Add reg_var, ZERO_REG, reg_result),
                ],
            )
        }
        Assign::Mod { var, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, *assign);
            let reg_assign = lir_assign.result_reg();

            let reg_var = ctx.ref_var_reg(&var).unwrap();

            let label_cond = ctx.alloc_label();
            let label_end = ctx.alloc_label();

            (
                reg_var,
                vec![
                    lir_assign,
                    lir!(Label label_cond),
                    lir!(Bne(18) ZERO_REG, reg_assign, ZERO_REG),
                    lir!(Add reg_var, ZERO_REG, ZERO_REG),
                    lir!(JmpLabel(label_end)),
                    lir!(Blt(12) ZERO_REG, reg_var, reg_assign),
                    lir!(Jmp(12)),
                    lir!(JmpLabel(label_end)),
                    lir!(Sub reg_var, reg_var, reg_assign),
                    lir!(JmpLabel(label_cond)),
                    lir!(Label label_end),
                ],
            )
        }
        Assign::ShiftL { var, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, *assign);
            let reg_result = lir_assign.result_reg();

            let reg_var = ctx.ref_var_reg(&var).unwrap();

            (reg_var, vec![lir_assign, lir!(ShiftL reg_var, reg_var, reg_result)])
        }
        Assign::ShiftR { var, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, *assign);
            let reg_result = lir_assign.result_reg();

            let reg_var = ctx.ref_var_reg(&var).unwrap();

            (reg_var, vec![lir_assign, lir!(ShiftR reg_var, reg_var, reg_result)])
        }
        Assign::ShiftRa { var, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, *assign);
            let reg_result = lir_assign.result_reg();

            let reg_var = ctx.ref_var_reg(&var).unwrap();

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
