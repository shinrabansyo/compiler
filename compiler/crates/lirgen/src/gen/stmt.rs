use sb_compiler_parse_ast::Stmt;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::{lir, LIR, Li, Add, Pop, FLoad, VarFree, Return};

use super::{RET_REG, TMP_REG};
use super::{lirgen_var_decl, lirgen_block, lirgen_expr, lirgen_if, lirgen_while, lirgen_for, lirgen_dev_io};

pub fn lirgen_stmt(lirs: &mut Vec<LIR>, stmt: &Stmt, analyze_result: &AnalyzeResult) {
    match stmt {
        Stmt::VarDecl { var_decl, .. } => {
            lirgen_var_decl(lirs, var_decl, analyze_result);
        }
        Stmt::Block { block, .. } => {
            lirgen_block(lirs, block, analyze_result);
        }
        Stmt::Expr { expr, .. } => {
            lirgen_expr(lirs, expr, analyze_result);
            lirs.push(lir!(Pop TMP_REG));
        }
        Stmt::Return { expr, .. } => {
            lirgen_expr(lirs, expr, analyze_result);
            lirs.push(lir!(Pop TMP_REG));
            lirs.push(lir!(Li RET_REG, 0));
            lirs.push(lir!(Add RET_REG, TMP_REG));
            lirs.push(lir!(VarFree));
            lirs.push(lir!(FLoad));
            lirs.push(lir!(Return));
        }
        Stmt::If { r#if, .. } => {
            lirgen_if(lirs, r#if, analyze_result);
        }
        Stmt::While { r#while, .. } => {
            lirgen_while(lirs, r#while, analyze_result);
        }
        Stmt::For { r#for, .. } => {
            lirgen_for(lirs, r#for, analyze_result);
        }
        Stmt::DevIO { dev_io, .. } => {
            lirgen_dev_io(lirs, dev_io, analyze_result);
        }
    }
}
