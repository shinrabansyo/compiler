use sb_compiler_lirgen_ir::LirTopElem;
use sb_compiler_semcheck_hir::Top;

use crate::GenContext;
use super::lirgen_func_def;

pub fn lirgen_top<'input>(ctx: &mut GenContext<'input>, top: &Top<'input>) -> LirTopElem {
    match top {
        Top::FuncDef { func_def, .. } => {
            lirgen_func_def(ctx, func_def)
        }
    }
}
