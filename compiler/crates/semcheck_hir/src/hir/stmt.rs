use sb_compiler_parse_ast as ast;

use super::{VarDecl, Block, Expr, Return, If, While, For, InlineAsm, SemCheckFrom, Dep};

#[derive(Debug)]
pub enum Stmt<'input> {
    VarDecl {
        var_decl: VarDecl<'input>,
    },
    Block {
        block: Block<'input>,
    },
    Expr {
        expr: Expr<'input>,
    },
    Return {
        r#return: Return<'input>,
    },
    If {
        r#if: If<'input>,
    },
    While {
        r#while: While<'input>,
    },
    For {
        r#for: For<'input>,
    },
    InlineAsm {
        inline_asm: InlineAsm,
    },
}

impl<'input> SemCheckFrom<Dep<'_>, ast::Stmt<'input>> for Stmt<'input> {
    async fn check0(ctx: Dep<'_>, stmt: ast::Stmt<'input>) -> anyhow::Result<Self>
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
                    inline_asm: InlineAsm::check(ctx, inline_asm).await?,
                })
            },
        }
    }
}
