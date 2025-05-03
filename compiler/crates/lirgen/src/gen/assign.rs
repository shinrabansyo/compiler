use sb_compiler_parse_ast::Assign;
use sb_compiler_lirgen_ir::{lir, LirTree, Add, Sub, ShiftL, ShiftR, ShiftRa};

use crate::{GenContext, ZERO_REG};
use super::lirgen_logic_or;

pub fn lirgen_assign(ctx: &mut GenContext, assign: &Assign) -> LirTree {
    let reserved_reg_range_start = ctx.reserved_regs;
    let reserved_label_range_start = ctx.reserved_labels;

    let lirs = match assign {
        Assign::Normal { ident, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, assign);
            let reg_result = lir_assign.reserved_reg_range().1 - 1;

            vec![
                lir_assign,
                lir!(Add *ctx.sym_table.get(ident).unwrap(), ZERO_REG, reg_result),
            ]
        }
        Assign::Plus { ident, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, assign);
            let reg_result = lir_assign.reserved_reg_range().1 - 1;

            let reg_var = *ctx.sym_table.get(ident).unwrap();

            vec![
                lir_assign,
                lir!(Add reg_var, reg_var, reg_result),
            ]
        }
        Assign::Minus { ident, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, assign);
            let reg_result = lir_assign.reserved_reg_range().1 - 1;

            let reg_var = *ctx.sym_table.get(ident).unwrap();

            vec![
                lir_assign,
                lir!(Sub reg_var, reg_var, reg_result),
            ]
        }
        Assign::ShiftL { ident, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, assign);
            let reg_result = lir_assign.reserved_reg_range().1 - 1;

            let reg_var = *ctx.sym_table.get(ident).unwrap();

            vec![
                lir_assign,
                lir!(ShiftL reg_var, reg_var, reg_result),
            ]
        }
        Assign::ShiftR { ident, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, assign);
            let reg_result = lir_assign.reserved_reg_range().1 - 1;

            let reg_var = *ctx.sym_table.get(ident).unwrap();

            vec![
                lir_assign,
                lir!(ShiftR reg_var, reg_var, reg_result),
            ]
        }
        Assign::ShiftRa { ident, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, assign);
            let reg_result = lir_assign.reserved_reg_range().1 - 1;

            let reg_var = *ctx.sym_table.get(ident).unwrap();

            vec![
                lir_assign,
                lir!(ShiftRa reg_var, reg_var, reg_result),
            ]
        }
        Assign::LogicOr { or, .. } => {
            return lirgen_logic_or(ctx, or);
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
