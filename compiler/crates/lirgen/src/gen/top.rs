use sb_compiler_lirgen_ir::LirTopElem;
use sb_compiler_semcheck_hir::Top;

use crate::GenContext;
use super::lirgen_func_def;

pub fn lirgen_top<'src>(ctx: &mut GenContext<'src>, top: &Top<'src>) -> LirTopElem {
    match top {
        Top::FuncDef { func_def, .. } => {
            lirgen_func_def(ctx, func_def)
        }
    }
}
