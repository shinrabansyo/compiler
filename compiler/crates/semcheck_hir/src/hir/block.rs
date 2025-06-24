use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_type::r#type::{Type, Void};
use sb_compiler_type::Typed;

use super::{Stmt, SemCheck, InDep};

#[derive(Debug)]
pub struct Block<'src> {
    pub span: Span<'src>,
    pub stmts: Vec<Stmt<'src>>,
}

impl<'src> SemCheck<InDep<'src>, ast::Block<'src>> for Block<'src> {
    async fn check0(mut ctx: InDep<'src>, block: ast::Block<'src>) -> miette::Result<Self>
    where
        Self: Sized,
    {
        // 文を順に意味解析
        let mut stmts = vec![];
        for stmt in block.stmts {
            stmts.push(Stmt::check(&mut ctx, stmt).await?);
        }

        Ok(Block { span: block.span, stmts })
    }
}

impl<'src> Spanned<'src> for Block<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}

impl Typed for Block<'_> {
    fn ty(&self) -> Arc<Type> {
        Void.ty()
    }
}
