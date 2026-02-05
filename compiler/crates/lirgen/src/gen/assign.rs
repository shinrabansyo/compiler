use sb_compiler_lirgen_ir::{lir, LirBlock, Add, Bne, Blt, Jmp, JmpLabel, Sub, Li,ShiftL, ShiftR, ShiftRa};
use sb_compiler_semcheck_hir::Assign;

use super::{GenContext, ZERO_REG, lirgen_logic_or, lirgen_value_l};

pub fn lirgen_assign<'src>(ctx: &mut GenContext<'src>, assign: Assign<'src>) -> LirBlock {
    let (result_reg, lirs) = match assign {
        Assign::Normal { lhs, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, *assign);
            let reg_assign = lir_assign.result_reg();

            let (_, lir_lhs) = lirgen_value_l(ctx, lhs);
            let reg_lhs = lir_lhs.result_reg();

            (
                reg_lhs,
                vec![
                    lir_assign,
                    lir!(Add reg_lhs, ZERO_REG, reg_assign),
                    lir_lhs,
                ],
            )
        }
        Assign::Plus { lhs, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, *assign);
            let reg_assign = lir_assign.result_reg();

            let lir_lhs = lirgen_value_l(ctx, lhs);
            let reg_lhs = (lir_lhs.0.result_reg(), lir_lhs.1.result_reg());

            (
                reg_lhs.1,
                vec![
                    lir_lhs.0,
                    lir_assign,
                    lir!(Add reg_lhs.1, reg_lhs.0, reg_assign),
                    lir_lhs.1,
                ],
            )
        }
        Assign::Minus { lhs, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, *assign);
            let reg_assign = lir_assign.result_reg();

            let lir_lhs = lirgen_value_l(ctx, lhs);
            let reg_lhs = (lir_lhs.0.result_reg(), lir_lhs.1.result_reg());

            (
                reg_lhs.1,
                vec![
                    lir_lhs.0,
                    lir_assign,
                    lir!(Sub reg_lhs.1, reg_lhs.0, reg_assign),
                    lir_lhs.1,
                ],
            )
        }
        Assign::Mul { lhs, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, *assign);
            let reg_assign = lir_assign.result_reg();

            let lir_lhs = lirgen_value_l(ctx, lhs);
            let reg_lhs = (lir_lhs.0.result_reg(), lir_lhs.1.result_reg());

            let reg_one = ctx.alloc_reg();
            let reg_cnt = ctx.alloc_reg();
            let reg_result = ctx.alloc_reg();

            let label_cond = ctx.alloc_label();
            let label_end = ctx.alloc_label();

            (
                reg_lhs.1,
                vec![
                    lir_lhs.0,
                    lir_assign,
                    lir!(Add reg_cnt, ZERO_REG, reg_assign),
                    lir!(Li(1) reg_one),
                    lir!(Li(0) reg_result),
                    lir!(Label label_cond),
                    lir!(Bne(12) ZERO_REG, reg_cnt, ZERO_REG),
                    lir!(JmpLabel(label_end)),
                    lir!(Add reg_result, reg_result, reg_lhs.0),
                    lir!(Sub reg_cnt, reg_cnt, reg_one),
                    lir!(JmpLabel(label_cond)),
                    lir!(Label label_end),
                    lir!(Add reg_lhs.1, ZERO_REG, reg_result),
                    lir_lhs.1,
                ],
            )
        }
        Assign::Div { lhs, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, *assign);
            let reg_assign = lir_assign.result_reg();

            let lir_lhs = lirgen_value_l(ctx, lhs);
            let reg_lhs = (lir_lhs.0.result_reg(), lir_lhs.1.result_reg());

            let reg_one = ctx.alloc_reg();
            let reg_result = ctx.alloc_reg();

            let label_cond = ctx.alloc_label();
            let label_end = ctx.alloc_label();

            (
                reg_lhs.1,
                vec![
                    lir_lhs.0,
                    lir_assign,
                    lir!(Li(1) reg_one),
                    lir!(Li(0) reg_result),
                    lir!(Label label_cond),
                    lir!(Bne(12) ZERO_REG, reg_assign, ZERO_REG),
                    lir!(JmpLabel(label_end)),
                    lir!(Blt(12) ZERO_REG, reg_lhs.0, reg_assign),
                    lir!(Jmp(12)),
                    lir!(JmpLabel(label_end)),
                    lir!(Add reg_result, reg_result, reg_one),
                    lir!(Sub reg_lhs.0, reg_lhs.0, reg_assign),
                    lir!(JmpLabel(label_cond)),
                    lir!(Label label_end),
                    lir!(Add reg_lhs.1, ZERO_REG, reg_result),
                    lir_lhs.1,
                ],
            )
        }
        Assign::Mod { lhs, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, *assign);
            let reg_assign = lir_assign.result_reg();

            let lir_lhs = lirgen_value_l(ctx, lhs);
            let reg_lhs = (lir_lhs.0.result_reg(), lir_lhs.1.result_reg());

            let label_cond = ctx.alloc_label();
            let label_end = ctx.alloc_label();

            (
                reg_lhs.1,
                vec![
                    lir_lhs.0,
                    lir_assign,
                    lir!(Label label_cond),
                    lir!(Add reg_lhs.1, ZERO_REG, reg_lhs.0),
                    lir!(Bne(18) ZERO_REG, reg_assign, ZERO_REG),
                    lir!(Add reg_lhs.1, ZERO_REG, ZERO_REG),
                    lir!(JmpLabel(label_end)),
                    lir!(Blt(12) ZERO_REG, reg_lhs.0, reg_assign),
                    lir!(Jmp(12)),
                    lir!(JmpLabel(label_end)),
                    lir!(Sub reg_lhs.1, reg_lhs.1, reg_assign),
                    lir!(JmpLabel(label_cond)),
                    lir!(Label label_end),
                    lir_lhs.1,
                ],
            )
        }
        Assign::ShiftL { lhs, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, *assign);
            let reg_result = lir_assign.result_reg();

            let lir_lhs = lirgen_value_l(ctx, lhs);
            let reg_lhs = (lir_lhs.0.result_reg(), lir_lhs.1.result_reg());

            (
                reg_lhs.1,
                vec![
                    lir_lhs.0,
                    lir_assign,
                    lir!(ShiftL reg_lhs.1, reg_lhs.0, reg_result),
                    lir_lhs.1,
                    ],
                )
        }
        Assign::ShiftR { lhs, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, *assign);
            let reg_result = lir_assign.result_reg();

            let lir_lhs = lirgen_value_l(ctx, lhs);
            let reg_lhs = (lir_lhs.0.result_reg(), lir_lhs.1.result_reg());

            (
                reg_lhs.1,
                vec![
                    lir_lhs.0,
                    lir_assign,
                    lir!(ShiftR reg_lhs.1, reg_lhs.0, reg_result),
                    lir_lhs.1,
                ],
            )
        }
        Assign::ShiftRa { lhs, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, *assign);
            let reg_result = lir_assign.result_reg();

            let lir_lhs = lirgen_value_l(ctx, lhs);
            let reg_lhs = (lir_lhs.0.result_reg(), lir_lhs.1.result_reg());

            (
                reg_lhs.1,
                vec![
                    lir_lhs.0,
                    lir_assign,
                    lir!(ShiftRa reg_lhs.1, reg_lhs.0, reg_result),
                    lir_lhs.1,
                ])
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
