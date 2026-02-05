use sb_compiler_lirgen_ir::{lir, LirBlock, Sb, Sh, Sw};
use sb_compiler_semcheck_hir::StructFieldInit;

use crate::GenContext;
use super::lirgen_expr;

pub fn lirgen_struct_field_init<'src>(
    ctx: &mut GenContext<'src>,
    struct_field_init: StructFieldInit<'src>,
    reg_addr: u32,
) -> LirBlock {
    let lir_expr = lirgen_expr(ctx, struct_field_init.expr);
    let reg_expr = lir_expr.result_reg();

    let offset = struct_field_init.offset as i32;
    let lir_write = match struct_field_init.size {
        1 => lir!(Sb(offset) 0, reg_addr, reg_expr),
        2 => lir!(Sh(offset) 0, reg_addr, reg_expr),
        4 => lir!(Sw(offset) 0, reg_addr, reg_expr),
        _ => unreachable!(),
    };

    LirBlock::Single {
        result_reg: reg_expr,
        lirs: vec![
            lir_expr,
            lir_write,
        ]
    }
}
