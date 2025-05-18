use sb_compiler_parse_ast::Stmt;
use sb_compiler_lirgen_ir::LirTree;

use crate::GenContext;
use super::{lirgen_var_decl, lirgen_block, lirgen_expr, lirgen_if, lirgen_while};

pub fn lirgen_stmt(ctx: &mut GenContext, stmt: &Stmt) -> LirTree {
    match stmt {
        Stmt::VarDecl { var_decl, .. } => {
            lirgen_var_decl(ctx, var_decl)
        }
        Stmt::Block { block, .. } => {
            lirgen_block(ctx, block)
        }
        Stmt::Expr { expr, .. } => {
            lirgen_expr(ctx, expr)
        }
        Stmt::Return { expr, .. } => {
            unimplemented!()
            // lirgen_expr(lirs, expr, analyze_result);
            // lirs.push(lir!(Pop TMP_REG));
            // lirs.push(lir!(Li RET_REG, 0));
            // lirs.push(lir!(Add RET_REG, TMP_REG));
            // lirs.push(lir!(VarFree));
            // lirs.push(lir!(FLoad));
            // lirs.push(lir!(Return));
        }
        Stmt::If { r#if, .. } => {
            lirgen_if(ctx, r#if)
        }
        Stmt::While { r#while, .. } => {
            lirgen_while(ctx, r#while)
        }
        Stmt::For { r#for, .. } => {
            unimplemented!()
            // lirgen_for(lirs, r#for, analyze_result);
        }
        Stmt::DevIO { dev_io, .. } => {
            unimplemented!()
            // lirgen_dev_io(lirs, dev_io, analyze_result);
        }
    }
}
