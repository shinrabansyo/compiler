use std::collections::VecDeque;

use sb_compiler_lirgen_ir::{lir, LirBlock, Addi};
use sb_compiler_semcheck_hir::StructInit;

use crate::GenContext;
use super::{ADDR_REG, lirgen_expr, lirgen_struct_field_init};

pub fn lirgen_struct_init<'src>(ctx: &mut GenContext<'src>, struct_init: StructInit<'src>) -> LirBlock {
    let lir_addr = lirgen_expr(ctx, *struct_init.addr);
    let reg_addr = lir_addr.result_reg();

    let mut lir_fields = struct_init
        .fields
        .into_iter()
        .map(|field| lirgen_struct_field_init(ctx, field))
        .collect::<VecDeque<_>>();

    lir_fields.push_front(lir!(Addi(0) ADDR_REG, reg_addr));
    lir_fields.push_front(lir_addr);

    LirBlock::Single {
        result_reg: reg_addr,
        lirs: lir_fields.into(),
    }
}
