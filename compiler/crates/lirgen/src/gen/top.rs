use sb_compiler_parse_ast::Top;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::LIR;

use super::{lirgen_const_decl, lirgen_func_def};

pub fn lirgen_top(lirs: &mut Vec<LIR>, top: &Top, analyze_result: &AnalyzeResult) {
    match top {
        Top::ConstDecl { const_decl, .. } => {
            lirgen_const_decl(lirs, const_decl, analyze_result);
        }
        Top::FuncDef { func_def, .. } => {
            lirgen_func_def(lirs, func_def, analyze_result);
        }
    }
}
