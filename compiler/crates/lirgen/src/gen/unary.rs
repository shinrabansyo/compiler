use sb_compiler_lirgen_ir::{lir, LirBlock, Beq, JmpLabel, Li, Sub};
use sb_compiler_semcheck_hir::Unary;

use super::{GenContext, ZERO_REG, lirgen_value_r};

pub fn lirgen_unary<'src>(ctx: &mut GenContext<'src>, unary: Unary<'src>) -> LirBlock {
    let (result_reg, lirs) = match unary {
        Unary::Not { value, .. } => {
            let lir_value = lirgen_value_r(ctx, value);
            let reg_value = lir_value.result_reg();

            let reg_result = ctx.alloc_reg();

            let label_false = ctx.alloc_label();
            let label_end = ctx.alloc_label();

            (
                reg_result,
                vec![
                    lir_value,
                    lir!(Beq(12) ZERO_REG, ZERO_REG, reg_value),
                    lir!(JmpLabel(label_false)),
                    lir!(Li(1) reg_result),
                    lir!(JmpLabel(label_end)),
                    lir!(Label label_false),
                    lir!(Li(0) reg_result),
                    lir!(Label label_end),
                ],
            )
        }
        Unary::Plus { value, .. } => {
            let lir_value = lirgen_value_r(ctx, value);
            let reg_value = lir_value.result_reg();

            (reg_value, vec![lir_value])
        }
        Unary::Minus { value, .. } => {
            let lir_value = lirgen_value_r(ctx, value);
            let reg_value = lir_value.result_reg();

            (
                reg_value,
                vec![
                    lir_value,
                    lir!(Sub reg_value, ZERO_REG, reg_value),
                ],
            )
        }
        Unary::Addr { value, ..  } => {
            let lir_value = lirgen_value_r(ctx, value);
            let reg_value = lir_value.result_reg();

            (reg_value, vec![lir_value])
        }
        Unary::SizeOf { size, .. } => {
            let reg_result = ctx.alloc_reg();

            (reg_result, vec![lir!(Li(size as i32) reg_result)])
        }
        Unary::ValueR { value, .. } => {
            return lirgen_value_r(ctx, value);
        }
    };

    LirBlock::Single {
        result_reg,
        lirs,
    }
}
