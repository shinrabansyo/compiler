use sb_compiler_parse_ast::Stmt;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::LIR;

use super::{lirgen_const_decl, lirgen_block, lirgen_expr};

pub fn lirgen_stmt(lirs: &mut Vec<LIR>, stmt: &Stmt, analyze_result: &AnalyzeResult) {
    match stmt {
        Stmt::ConstDecl { const_decl, .. } => {
            lirgen_const_decl(lirs, const_decl, analyze_result);
        }
        Stmt::Block { block, .. } => {
            lirgen_block(lirs, block, analyze_result);
        }
        Stmt::Expr { expr, .. } => {
            lirgen_expr(lirs, expr, analyze_result);
        }
    }
}
