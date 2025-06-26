use sb_compiler_lirgen_ir::LirBlock;
use sb_compiler_semcheck_hir::Cast;

use crate::GenContext;
use super::lirgen_unary;

pub fn lirgen_cast(ctx: &mut GenContext, cast: Cast) -> LirBlock {
    match cast {
        Cast::Casting { unary, .. } => lirgen_unary(ctx, unary),
        Cast::Unary { unary} => lirgen_unary(ctx, unary),
    }
}
