use sb_compiler_lirgen_ir::{lir, LirBlock, Sub};
use sb_compiler_semcheck_hir::Unary;

use crate::{GenContext, ZERO_REG};
use super::lirgen_value;

pub fn lirgen_unary(ctx: &mut GenContext, unary: &Unary) -> LirBlock {
    let (result_reg, lirs) = match unary {
        Unary::Plus { value, .. } => {
            let lir_value = lirgen_value(ctx, value);
            let reg_value = lir_value.result_reg();

            (reg_value, vec![lir_value])
        }
        Unary::Minus { value, .. } => {
            let lir_value = lirgen_value(ctx, value);
            let reg_value = lir_value.result_reg();

            (
                reg_value,
                vec![
                    lir_value,
                    lir!(Sub reg_value, ZERO_REG, reg_value),
                ],
            )
        }
        Unary::Value { value, .. } => {
            return lirgen_value(ctx, value);
        }
    };

    LirBlock::Single {
        result_reg,
        lirs,
    }
}
