use sb_compiler_parse_ast::LogicOr;
use sb_compiler_lirgen_ir::{lir, LirBlock, Beq, JmpLabel, Li};

use crate::{GenContext, ZERO_REG};
use super::lirgen_logic_and;

pub fn lirgen_logic_or(ctx: &mut GenContext, logic_or: &LogicOr) -> LirBlock {
    let (result_reg, lirs) = match logic_or {
        LogicOr::Or { lhs, rhs, .. } => {
            let lir_lhs = lirgen_logic_or(ctx, lhs);
            let reg_lhs = lir_lhs.result_reg();

            let lir_rhs = lirgen_logic_and(ctx, rhs);
            let reg_rhs = lir_rhs.result_reg();

            let reg_result = ctx.alloc_reg();

            let label_true = ctx.alloc_label();
            let label_end = ctx.alloc_label();

            (
                reg_result,
                vec![
                    lir_lhs,
                    lir!(Beq(12) ZERO_REG, ZERO_REG, reg_lhs),
                    lir!(JmpLabel(label_true)),

                    lir_rhs,
                    lir!(Beq(12) ZERO_REG, ZERO_REG, reg_rhs),
                    lir!(JmpLabel(label_true)),

                    lir!(Li(0) reg_result),
                    lir!(JmpLabel(label_end)),

                    lir!(Label label_true),
                    lir!(Li(1) reg_result),

                    lir!(Label label_end),
                ],
            )
        }
        LogicOr::LogicAnd { and, .. } => {
            return lirgen_logic_and(ctx, and);
        }
    };

    LirBlock::Single {
        result_reg,
        lirs,
    }
}
