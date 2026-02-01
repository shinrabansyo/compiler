use sb_compiler_lirgen_ir::{lir, LirBlock, Add, Lb, Lh, Lw};
use sb_compiler_semcheck_hir::StructAccess;

use crate::GenContext;
use super::{ZERO_REG, ADDR_REG, lirgen_value};

pub fn lirgen_struct_access<'src>(ctx: &mut GenContext<'src>, struct_access: StructAccess<'src>) -> LirBlock {
    let lir_lhs = lirgen_value(ctx, *struct_access.target);
    let reg_lhs = lir_lhs.result_reg();

    let lir_addr = lir!(Add ADDR_REG, ZERO_REG, reg_lhs);

    let offset = struct_access.offset as i32;
    let reg_read = ctx.alloc_reg();
    let lir_read = match struct_access.ty.size() {
        1 => lir!(Lb(offset) reg_read, ADDR_REG),
        2 => lir!(Lh(offset) reg_read, ADDR_REG),
        4 => lir!(Lw(offset) reg_read, ADDR_REG),
        _ => unreachable!(),
    };

    LirBlock::Single {
        result_reg: reg_read,
        lirs: vec![
            lir_lhs,
            lir_addr,
            lir_read,
        ],
    }
}
