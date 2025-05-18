use sb_compiler_parse_ast::Stmt;
use sb_compiler_lirgen_ir::{lir, LirBlock, Add};

use crate::{GenContext, RET_REG, ZERO_REG};
use super::{lirgen_var_decl, lirgen_block, lirgen_expr, lirgen_if, lirgen_while, lirgen_for};

pub fn lirgen_stmt(ctx: &mut GenContext, stmt: &Stmt) -> LirBlock {
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
            // 式
            let lir_expr = lirgen_expr(ctx, expr);
            let reg_expr = lir_expr.result_reg();

            LirBlock::Single {
                result_reg: ZERO_REG,
                lirs: vec![
                    lir_expr,
                    lir!(Add RET_REG, ZERO_REG, reg_expr),
                ],
            }
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
        Stmt::DevIO { dev_io, .. } => {
            unimplemented!()
            // lirgen_dev_io(lirs, dev_io, analyze_result);
        }
    }
}
