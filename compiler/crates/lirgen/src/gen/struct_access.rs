use sb_compiler_lirgen_ir::{lir, LirBlock, Add, Li};
use sb_compiler_semcheck_hir::StructAccess;

use crate::GenContext;
use super::{ADDR_REG, lirgen_value_r};

pub fn lirgen_struct_access<'src>(ctx: &mut GenContext<'src>, struct_access: StructAccess<'src>) -> LirBlock {
    let lir_lhs = lirgen_value_r(ctx, *struct_access.target);
    let reg_lhs = lir_lhs.result_reg();

    let offset = struct_access.offset as i32;

    LirBlock::Single {
        result_reg: ADDR_REG,
        lirs: vec![
            lir_lhs,
            lir!(Li(offset) ADDR_REG),
            lir!(Add ADDR_REG, ADDR_REG, reg_lhs),
        ],
    }
}
