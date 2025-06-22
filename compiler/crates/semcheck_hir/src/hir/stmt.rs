use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_type::r#type::{Primitive, Type, Void};
use sb_compiler_type::Typed;

use super::{VarDecl, Block, Expr, Return, If, While, For, InlineAsm, SemCheck, Dep};

#[derive(Debug)]
pub enum Stmt<'src> {
    VarDecl {
        var_decl: VarDecl<'src>,
        ty: Arc<Type>,
    },
    Block {
        block: Block<'src>,
        ty: Arc<Type>,
    },
    Expr {
        expr: Expr<'src>,
        ty: Arc<Type>,
    },
    Return {
        r#return: Return<'src>,
        ty: Arc<Type>,
    },
    If {
        r#if: If<'src>,
        ty: Arc<Type>,
    },
    While {
        r#while: While<'src>,
        ty: Arc<Type>,
    },
    For {
        r#for: For<'src>,
        ty: Arc<Type>,
    },
    InlineAsm {
        inline_asm: InlineAsm<'src>,
        ty: Arc<Type>,
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
                    ty: Arc::new(Primitive(Void)),
                })
            },
            ast::Stmt::Block { block } => {
                Ok(Stmt::Block {
                    block: Block::check(ctx.clone(), block).await?,
                    ty: Arc::new(Primitive(Void)),
                })
            },
            ast::Stmt::Expr { expr } => {
                Ok(Stmt::Expr {
                    expr: Expr::check(ctx, expr).await?,
                    ty: Arc::new(Primitive(Void)),
                })
            },
            ast::Stmt::Return { r#return } => {
                Ok(Stmt::Return {
                    r#return: Return::check(ctx, r#return).await?,
                    ty: Arc::new(Primitive(Void)),
                })
            },
            ast::Stmt::If { r#if } => {
                Ok(Stmt::If {
                    r#if: If::check(ctx, r#if).await?,
                    ty: Arc::new(Primitive(Void)),
                })
            },
            ast::Stmt::While { r#while } => {
                Ok(Stmt::While {
                    r#while: While::check(ctx, r#while).await?,
                    ty: Arc::new(Primitive(Void)),
                })
            },
            ast::Stmt::For { r#for } => {
                Ok(Stmt::For {
                    r#for: For::check(ctx.clone(), r#for).await?,
                    ty: Arc::new(Primitive(Void)),
                })
            },
            ast::Stmt::InlineAsm { inline_asm } => {
                Ok(Stmt::InlineAsm {
                    inline_asm: InlineAsm::check(ctx.clone(), inline_asm).await?,
                    ty: Arc::new(Primitive(Void)),
                })
            },
        }
    }
}

impl Typed for Stmt<'_> {
    fn ty(&self) -> &Arc<Type> {
        match self {
            Stmt::VarDecl { ty, .. } => ty,
            Stmt::Block { ty, .. } => ty,
            Stmt::Expr { ty, .. } => ty,
            Stmt::Return { ty, .. } => ty,
            Stmt::If { ty, .. } => ty,
            Stmt::While { ty, .. } => ty,
            Stmt::For { ty, .. } => ty,
            Stmt::InlineAsm { ty, .. } => ty,
        }
    }
}
