use sb_compiler_lirgen_ir::{lir, Lb, Lh, Lw, Sb, Sh, Sw, LirBlock};
use sb_compiler_semcheck_hir::ValueL;

use super::{GenContext, lirgen_struct_access};

pub fn lirgen_value_l<'src>(ctx: &mut GenContext<'src>, value: ValueL<'src>) -> (LirBlock, LirBlock) {
    match value {
        ValueL::Var { var, .. } => {
            let reg_var = ctx.ref_var_reg(&var).unwrap();
            let lir_read = LirBlock::Single {
                result_reg: reg_var,
                lirs: vec![],
            };
            let lir_write = LirBlock::Single {
                result_reg: reg_var,
                lirs: vec![],
            };

            (lir_read, lir_write)
        }
        ValueL::StructAccess { struct_access } => {
            let size = struct_access.ty.size();

            let lir_addr = lirgen_struct_access(ctx, struct_access);
            let reg_addr = lir_addr.result_reg();

            let reg_read = ctx.alloc_reg();
            let lir_read = match size {
                1 => lir!(Lb(0) reg_read, reg_addr),
                2 => lir!(Lh(0) reg_read, reg_addr),
                4 => lir!(Lw(0) reg_read, reg_addr),
                _ => unreachable!(),
            };
            let lir_read = LirBlock::Single {
                result_reg: reg_read,
                lirs: vec![lir_addr, lir_read],
            };

            let reg_write = ctx.alloc_reg();
            let lir_write = match size {
                1 => lir!(Sb(0) reg_addr, reg_write),
                2 => lir!(Sh(0) reg_addr, reg_write),
                4 => lir!(Sw(0) reg_addr, reg_write),
                _ => unreachable!(),
            };
            let lir_write = LirBlock::Single {
                result_reg: reg_write,
                lirs: vec![lir_write],
            };

            (lir_read, lir_write)
        }
    }
}
