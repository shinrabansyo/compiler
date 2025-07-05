use sb_compiler_lirgen_ir::LirBlock;
use sb_compiler_semcheck_hir::Stmt;

use crate::GenContext;
use super::{
    lirgen_var_decl, lirgen_block, lirgen_expr, lirgen_return,
    lirgen_if, lirgen_while, lirgen_for, lirgen_inline_asm
};

pub fn lirgen_stmt<'src>(ctx: &mut GenContext<'src>, stmt: Stmt<'src>) -> LirBlock {
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
        Stmt::Return { r#return, .. } => {
            lirgen_return(ctx, r#return)
        }
        Stmt::If { r#if, .. } => {
            lirgen_if(ctx, r#if)
        }
        Stmt::While { r#while, .. } => {
            lirgen_while(ctx, r#while)
        }
        Stmt::For { r#for, .. } => {
            lirgen_for(ctx, r#for)
        }
        Stmt::InlineAsm { inline_asm, .. } => {
            lirgen_inline_asm(ctx, inline_asm)
        }
    }
}
