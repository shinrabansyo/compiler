use sb_compiler_lirgen_ir::{lir, LirBlock, Add, Blt, Bne, Jmp, JmpLabel, Li, Sub};
use sb_compiler_semcheck_hir::Mul as MulHir;

use crate::{GenContext, ZERO_REG};
use super::lirgen_cast;

pub fn lirgen_mul(ctx: &mut GenContext, add: &MulHir) -> LirBlock {
    let (result_reg, lirs) = match add {
        MulHir::Multiply { lhs, rhs, .. } => {
            let lir_lhs = lirgen_mul(ctx, lhs);
            let reg_lhs = lir_lhs.result_reg();

            let lir_rhs = lirgen_cast(ctx, rhs);
            let reg_rhs = lir_rhs.result_reg();

            let reg_one = ctx.alloc_reg();
            let reg_cnt = ctx.alloc_reg();
            let reg_result = ctx.alloc_reg();

            let label_cond = ctx.alloc_label();
            let label_end = ctx.alloc_label();

            (
                reg_result,
                vec![
                    lir_lhs,
                    lir_rhs,
                    lir!(Add reg_cnt, ZERO_REG, reg_rhs),
                    lir!(Li(1) reg_one),
                    lir!(Li(0) reg_result),
                    lir!(Label label_cond),
                    lir!(Bne(12) ZERO_REG, reg_cnt, ZERO_REG),
                    lir!(JmpLabel(label_end)),
                    lir!(Add reg_result, reg_result, reg_lhs),
                    lir!(Sub reg_cnt, reg_cnt, reg_one),
                    lir!(JmpLabel(label_cond)),
                    lir!(Label label_end),
                ],
            )
        }
        MulHir::Divide { lhs, rhs, .. } => {
            let lir_lhs = lirgen_mul(ctx, lhs);
            let reg_lhs = lir_lhs.result_reg();

            let lir_rhs = lirgen_cast(ctx, rhs);
            let reg_rhs = lir_rhs.result_reg();

            let reg_one = ctx.alloc_reg();
            let reg_result = ctx.alloc_reg();

            let label_cond = ctx.alloc_label();
            let label_end = ctx.alloc_label();

            (
                reg_result,
                vec![
                    lir_lhs,
                    lir_rhs,
                    lir!(Li(1) reg_one),
                    lir!(Li(0) reg_result),
                    lir!(Label label_cond),
                    lir!(Bne(12) ZERO_REG, reg_rhs, ZERO_REG),
                    lir!(JmpLabel(label_end)),
                    lir!(Blt(12) ZERO_REG, reg_lhs, reg_rhs),
                    lir!(Jmp(12)),
                    lir!(JmpLabel(label_end)),
                    lir!(Add reg_result, reg_result, reg_one),
                    lir!(Sub reg_lhs, reg_lhs, reg_rhs),
                    lir!(JmpLabel(label_cond)),
                    lir!(Label label_end),
                ],
            )
        }
        MulHir::Modulo { lhs, rhs, .. } => {
            let lir_lhs = lirgen_mul(ctx, lhs);
            let reg_lhs = lir_lhs.result_reg();

            let lir_rhs = lirgen_cast(ctx, rhs);
            let reg_rhs = lir_rhs.result_reg();

            let label_cond = ctx.alloc_label();
            let label_end = ctx.alloc_label();

            (
                reg_lhs,
                vec![
                    lir_lhs,
                    lir_rhs,
                    lir!(Label label_cond),
                    lir!(Bne(18) ZERO_REG, reg_rhs, ZERO_REG),
                    lir!(Add reg_lhs, ZERO_REG, ZERO_REG),
                    lir!(JmpLabel(label_end)),
                    lir!(Blt(12) ZERO_REG, reg_lhs, reg_rhs),
                    lir!(Jmp(12)),
                    lir!(JmpLabel(label_end)),
                    lir!(Sub reg_lhs, reg_lhs, reg_rhs),
                    lir!(JmpLabel(label_cond)),
                    lir!(Label label_end),
                ],
            )
        }
        MulHir::Cast { value, .. } => {
            return lirgen_cast(ctx, value);
        }
    };

    LirBlock::Single {
        result_reg,
        lirs,
    }
}
