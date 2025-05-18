use sb_compiler_parse_ast::Top;
use sb_compiler_lirgen_ir::LirTopElem;

use crate::GenContext;
use super::{lirgen_var_decl, lirgen_func_def};

pub fn lirgen_top(ctx: &mut GenContext, top: &Top) -> LirTopElem {
    match top {
        Top::VarDecl { var_decl, .. } => {
            todo!()
            // lirgen_var_decl(ctx, var_decl)
        }
        Top::FuncDef { func_def, .. } => {
            lirgen_func_def(ctx, func_def)
        }
    }
}
