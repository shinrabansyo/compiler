use sb_compiler_parse_ast::Top;
use sb_compiler_lirgen_ir::LirTopElem;

use crate::GenContext;
use super::lirgen_func_def;

pub fn lirgen_top<'input>(ctx: &mut GenContext<'input>, top: &Top<'input>) -> LirTopElem {
    match top {
        Top::FuncDef { func_def, .. } => {
            lirgen_func_def(ctx, func_def)
        }
    }
}
