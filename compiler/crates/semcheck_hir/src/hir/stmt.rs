use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_type::r#type::Type;
use sb_compiler_type::Typed;

use super::{VarDecl, Block, Expr, Return, If, While, For, InlineAsm, SemCheck, Dep};

#[derive(Debug)]
pub enum Stmt<'src> {
    VarDecl {
        var_decl: VarDecl<'src>,
    },
    Block {
        block: Block<'src>,
    },
    Expr {
        expr: Expr<'src>,
    },
    Return {
        r#return: Return<'src>,
    },
    If {
        r#if: If<'src>,
    },
    While {
        r#while: While<'src>,
    },
    For {
        r#for: For<'src>,
    },
    InlineAsm {
        inline_asm: InlineAsm<'src>,
    },
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::Stmt<'src>> for Stmt<'src> {
    async fn check0(ctx: Dep<'_, 'src>, stmt: ast::Stmt<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match stmt {
            ast::Stmt::VarDecl { var_decl } => {
                Ok(Stmt::VarDecl {
                    var_decl: VarDecl::check(ctx, var_decl).await?,
                })
            },
            ast::Stmt::Block { block } => {
                Ok(Stmt::Block {
                    block: Block::check(ctx.clone(), block).await?,
                })
            },
            ast::Stmt::Expr { expr } => {
                Ok(Stmt::Expr {
                    expr: Expr::check(ctx, expr).await?,
                })
            },
            ast::Stmt::Return { r#return } => {
                Ok(Stmt::Return {
                    r#return: Return::check(ctx, r#return).await?,
                })
            },
            ast::Stmt::If { r#if } => {
                Ok(Stmt::If {
                    r#if: If::check(ctx, r#if).await?,
                })
            },
            ast::Stmt::While { r#while } => {
                Ok(Stmt::While {
                    r#while: While::check(ctx, r#while).await?,
                })
            },
            ast::Stmt::For { r#for } => {
                Ok(Stmt::For {
                    r#for: For::check(ctx, r#for).await?,
                })
            },
            ast::Stmt::InlineAsm { inline_asm } => {
                Ok(Stmt::InlineAsm {
                    inline_asm: InlineAsm::check(ctx.clone(), inline_asm).await?,
                })
            },
        }
    }
}

impl Typed for Stmt<'_> {
    fn ty(&self) -> &Arc<Type> {
        match self {
            Stmt::VarDecl { var_decl } => var_decl.ty(),
            Stmt::Block { block } => block.ty(),
            Stmt::Expr { expr } => expr.ty(),
            Stmt::Return { r#return } => r#return.ty(),
            Stmt::If { r#if } => r#if.ty(),
            Stmt::While { r#while } => r#while.ty(),
            Stmt::For { r#for } => r#for.ty(),
            Stmt::InlineAsm { inline_asm } => inline_asm.ty(),
        }
    }
}
