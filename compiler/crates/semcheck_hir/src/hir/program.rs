use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_semcheck_async::prelude::*;
use sb_compiler_semcheck_impl_type::{Typed, Type, Void};
use sb_compiler_utils::error::ErrComposer;

use super::{Top, SemCheck, InDep};

#[derive(Debug)]
pub struct Program<'src> {
    pub span: Span<'src>,
    pub top_elems: Vec<Top<'src>>,
}

impl<'src> SemCheck<InDep<'src>, ast::Program<'src>> for Program<'src> {
    async fn check0(ctx: InDep<'src>, program: ast::Program<'src>) -> miette::Result<Self>
    where
        Self: Sized,
    {
        // トップ要素を意味解析
        let top_elems = program
            .top_elems
            .into_iter()
            .map(|top| Top::check(ctx.clone(), top))
            .join_all()
            .await
            .compose()?;

        Ok(Program { span: program.span, top_elems })
    }
}

impl<'src> Spanned<'src> for Program<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}

impl Typed for Program<'_> {
    fn ty(&self) -> Arc<Type> {
        Void.ty()
    }
}
