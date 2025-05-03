use sb_compiler_parse_ast::Top;
use sb_compiler_lirgen_ir::LirTree;

use crate::GenContext;
// use super::{lirgen_var_decl, lirgen_func_def};
use super::lirgen_var_decl;

pub fn lirgen_top(ctx: &mut GenContext, top: &Top) -> LirTree {
    let lir = match top {
        Top::VarDecl { var_decl, .. } => {
            lirgen_var_decl(ctx, var_decl)
        }
        _ => todo!(),
        // Top::FuncDef { func_def, .. } => {
        //     lirgen_func_def(&mut context, func_def);
        // }
    };

    LirTree::Node {
        reserved_reg_range: lir.reserved_reg_range(),
        reserved_label_range: lir.reserved_label_range(),
        lirs: vec![lir],
    }
}
