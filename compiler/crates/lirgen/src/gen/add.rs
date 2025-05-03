use sb_compiler_parse_ast::Add as AddAst;
use sb_compiler_lirgen_ir::{lir, LirTree, Add, Sub};

use crate::GenContext;
use super::lirgen_value;

pub fn lirgen_add(ctx: &mut GenContext, add: &AddAst) -> LirTree {
    let reserved_reg_range_start = ctx.reserved_regs;
    let reserved_label_range_start = ctx.reserved_labels;

    let lirs = match add {
        AddAst::Plus { lhs, rhs, .. } => {
            let lir_lhs = lirgen_add(ctx, lhs);
            let reg_lhs = lir_lhs.reserved_reg_range().1 - 1;

            let lir_rhs = lirgen_value(ctx, rhs);
            let reg_rhs = lir_rhs.reserved_reg_range().1 - 1;

            vec![
                lir_lhs,
                lir_rhs,
                lir!(Add ctx.alloc_reg(), reg_lhs, reg_rhs),
            ]
        }
        AddAst::Minus { lhs, rhs, .. } => {
            let lir_lhs = lirgen_add(ctx, lhs);
            let reg_lhs = lir_lhs.reserved_reg_range().1 - 1;

            let lir_rhs = lirgen_value(ctx, rhs);
            let reg_rhs = lir_rhs.reserved_reg_range().1 - 1;

            vec![
                lir_lhs,
                lir_rhs,
                lir!(Sub ctx.alloc_reg(), reg_lhs, reg_rhs),
            ]
        }
        AddAst::Value { value, .. } => {
            return lirgen_value(ctx, value);
        }
    };

    let reserved_reg_range = (reserved_reg_range_start, ctx.reserved_regs);
    let reserved_label_range = (reserved_label_range_start, ctx.reserved_labels);

    LirTree::Node {
        reserved_reg_range,
        reserved_label_range,
        lirs,
    }
}
