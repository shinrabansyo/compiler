use sb_compiler_lirgen_ir::{lir, LirBlock, Add, Li};
use sb_compiler_semcheck_hir::StructAccess;

use crate::GenContext;
use super::lirgen_value_r;

pub fn lirgen_struct_access<'src>(ctx: &mut GenContext<'src>, struct_access: StructAccess<'src>) -> LirBlock {
    let lir_lhs = lirgen_value_r(ctx, *struct_access.target);
    let reg_lhs = lir_lhs.result_reg();

    let offset = struct_access.offset as i32;
    let reg_offset = ctx.alloc_reg();

    LirBlock::Single {
        result_reg: reg_lhs,
        lirs: vec![
            lir_lhs,
            lir!(Li(offset) reg_offset),
            lir!(Add reg_lhs, reg_lhs, reg_offset),
        ],
    }
}
