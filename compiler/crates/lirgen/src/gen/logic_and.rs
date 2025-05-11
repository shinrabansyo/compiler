use sb_compiler_parse_ast::LogicAnd;
use sb_compiler_lirgen_ir::{lir, LirTree, Bne, JmpLabel, Li};

use crate::{GenContext, ZERO_REG};
use super::lirgen_bit_or;

pub fn lirgen_logic_and(ctx: &mut GenContext, logic_and: &LogicAnd) -> LirTree {
    let reserved_reg_range_start = ctx.reserved_regs;
    let reserved_label_range_start = ctx.reserved_labels;

    let (result_reg, lirs) = match logic_and {
        LogicAnd::And { lhs, rhs, .. } => {
            let lir_lhs = lirgen_logic_and(ctx, lhs);
            let reg_lhs = lir_lhs.result_reg();

            let lir_rhs = lirgen_bit_or(ctx, rhs);
            let reg_rhs = lir_rhs.result_reg();

            let reg_result = ctx.alloc_reg();

            let label_false = ctx.alloc_label();
            let label_end = ctx.alloc_label();

            (
                reg_result,
                vec![
                    lir_lhs,
                    lir!(Bne(12) ZERO_REG, ZERO_REG, reg_lhs),
                    lir!(JmpLabel(label_false)),

                    lir_rhs,
                    lir!(Bne(12) ZERO_REG, ZERO_REG, reg_rhs),
                    lir!(JmpLabel(label_false)),

                    lir!(Li(1) reg_result),
                    lir!(JmpLabel(label_end)),

                    lir!(Label label_false),
                    lir!(Li(0) reg_result),

                    lir!(Label label_end),
                ],
            )
        }
        LogicAnd::BitOr { or, .. } => {
            return lirgen_bit_or(ctx, or);
        }
    };

    let reserved_reg_range = (reserved_reg_range_start, ctx.reserved_regs);
    let reserved_label_range = (reserved_label_range_start, ctx.reserved_labels);

    LirTree::Node {
        reserved_reg_range,
        reserved_label_range,
        result_reg,
        lirs,
    }
}
