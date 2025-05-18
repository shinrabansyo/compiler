use sb_compiler_parse_ast::FuncDef;
use sb_compiler_lirgen_ir::LirTopElem;

use crate::GenContext;
use super::lirgen_block;

pub fn lirgen_func_def(ctx: &mut GenContext, func: &FuncDef) -> LirTopElem {
    LirTopElem::Function {
        namespace: func.namespace.clone(),
        name: func.ident.clone(),
        body: lirgen_block(ctx, &func.block),
    }
}
