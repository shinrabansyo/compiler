use sb_compiler_parse_ast::Assign;
use sb_compiler_lirgen_ir::{lir, LirBlock, Add, Sub, ShiftL, ShiftR, ShiftRa};

use crate::{GenContext, ZERO_REG};
use super::lirgen_logic_or;

pub fn lirgen_assign(ctx: &mut GenContext, assign: &Assign) -> LirBlock {
    let (result_reg, lirs) = match assign {
        Assign::Normal { ident, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, assign);
            let reg_assign = lir_assign.result_reg();

            let reg_var = *ctx.sym_table.get(ident).unwrap();

            (reg_var, vec![lir_assign, lir!(Add reg_var, ZERO_REG, reg_assign)])
        }
        Assign::Plus { ident, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, assign);
            let reg_assign = lir_assign.result_reg();

            let reg_var = *ctx.sym_table.get(ident).unwrap();

            (reg_var, vec![lir_assign, lir!(Add reg_var, reg_var, reg_assign)])
        }
        Assign::Minus { ident, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, assign);
            let reg_assign = lir_assign.result_reg();

            let reg_var = *ctx.sym_table.get(ident).unwrap();

            (reg_var, vec![lir_assign, lir!(Sub reg_var, reg_var, reg_assign)])
        }
        Assign::ShiftL { ident, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, assign);
            let reg_result = lir_assign.result_reg();

            let reg_var = *ctx.sym_table.get(ident).unwrap();

            (reg_var, vec![lir_assign, lir!(ShiftL reg_var, reg_var, reg_result)])
        }
        Assign::ShiftR { ident, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, assign);
            let reg_result = lir_assign.result_reg();

            let reg_var = *ctx.sym_table.get(ident).unwrap();

            (reg_var, vec![lir_assign, lir!(ShiftR reg_var, reg_var, reg_result)])
        }
        Assign::ShiftRa { ident, assign, .. } => {
            let lir_assign = lirgen_assign(ctx, assign);
            let reg_result = lir_assign.result_reg();

            let reg_var = *ctx.sym_table.get(ident).unwrap();

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
