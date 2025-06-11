use sb_compiler_parse_ast::Add as AddAst;
use sb_compiler_lirgen_ir::{lir, LirBlock, Add, Sub};

use crate::GenContext;
use super::lirgen_unary;

pub fn lirgen_add(ctx: &mut GenContext, add: &AddAst) -> LirBlock {
    let (result_reg, lirs) = match add {
        AddAst::Plus { lhs, rhs, .. } => {
            let lir_lhs = lirgen_add(ctx, lhs);
            let reg_lhs = lir_lhs.result_reg();

            let lir_rhs = lirgen_unary(ctx, rhs);
            let reg_rhs = lir_rhs.result_reg();

            let reg_result = ctx.alloc_reg();

            (
                reg_result,
                vec![
                    lir_lhs,
                    lir_rhs,
                    lir!(Add reg_result, reg_lhs, reg_rhs),
                ],
            )
        }
        AddAst::Minus { lhs, rhs, .. } => {
            let lir_lhs = lirgen_add(ctx, lhs);
            let reg_lhs = lir_lhs.result_reg();

            let lir_rhs = lirgen_unary(ctx, rhs);
            let reg_rhs = lir_rhs.result_reg();

            let reg_result = ctx.alloc_reg();

            (
                reg_result,
                vec![
                    lir_lhs,
                    lir_rhs,
                    lir!(Sub reg_result, reg_lhs, reg_rhs),
                ],
            )
        }
        AddAst::Unary { value, .. } => {
            return lirgen_unary(ctx, value);
        }
    };

    LirBlock::Single {
        result_reg,
        lirs,
    }
}
