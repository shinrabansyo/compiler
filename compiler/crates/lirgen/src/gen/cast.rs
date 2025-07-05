use sb_compiler_lirgen_ir::LirBlock;
use sb_compiler_semcheck_hir::Cast;

use crate::GenContext;
use super::lirgen_unary;

pub fn lirgen_cast<'src>(ctx: &mut GenContext<'src>, cast: Cast<'src>) -> LirBlock {
    match cast {
        Cast::Casting { unary, .. } => lirgen_unary(ctx, unary),
        Cast::Unary { unary} => lirgen_unary(ctx, unary),
    }
}
