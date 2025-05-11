use sb_compiler_parse_ast::BitXor;
use sb_compiler_lirgen_ir::{lir, LirTree, Xor};

use crate::GenContext;
use super::lirgen_bit_and;

pub fn lirgen_bit_xor(ctx: &mut GenContext, bit_xor: &BitXor) -> LirTree {
    let reserved_reg_range_start = ctx.reserved_regs;
    let reserved_label_range_start = ctx.reserved_labels;

    let (result_reg, lirs) = match bit_xor {
        BitXor::Xor { lhs, rhs, .. } => {
            let lir_lhs = lirgen_bit_xor(ctx, lhs);
            let reg_lhs = lir_lhs.result_reg();

            let lir_rhs = lirgen_bit_and(ctx, rhs);
            let reg_rhs = lir_rhs.result_reg();

            let reg_result = ctx.alloc_reg();

            (
                reg_result,
                vec![
                    lir_lhs,
                    lir_rhs,
                    lir!(Xor reg_result, reg_lhs, reg_rhs),
                ],
            )
        }
        BitXor::BitAnd { and, .. } => {
            return lirgen_bit_and(ctx, and);
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
