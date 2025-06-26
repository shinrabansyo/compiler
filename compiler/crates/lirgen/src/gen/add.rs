use sb_compiler_lirgen_ir::{lir, LirBlock, Add, Sub};
use sb_compiler_semcheck_hir::Add as AddHir;

use crate::GenContext;
use super::lirgen_mul;

pub fn lirgen_add(ctx: &mut GenContext, add: AddHir) -> LirBlock {
    let (result_reg, lirs) = match add {
        AddHir::Plus { lhs, rhs, .. } => {
            let lir_lhs = lirgen_add(ctx, *lhs);
            let reg_lhs = lir_lhs.result_reg();

            let lir_rhs = lirgen_mul(ctx, rhs);
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
        AddHir::Minus { lhs, rhs, .. } => {
            let lir_lhs = lirgen_add(ctx, *lhs);
            let reg_lhs = lir_lhs.result_reg();

            let lir_rhs = lirgen_mul(ctx, rhs);
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
        AddHir::Mul { value, .. } => {
            return lirgen_mul(ctx, value);
        }
    };

    LirBlock::Single {
        result_reg,
        lirs,
    }
}
