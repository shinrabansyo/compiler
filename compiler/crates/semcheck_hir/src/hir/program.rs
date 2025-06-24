use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::Span;
use sb_compiler_semcheck_async::prelude::*;
use sb_compiler_type::r#type::{Primitive, Type, Void};
use sb_compiler_type::Typed;
use sb_compiler_utils::error::ErrComposer;

use super::{Top, SemCheck, InDep};

#[derive(Debug)]
pub struct Program<'src> {
    pub span: Span<'src>,
    pub top_elems: Vec<Top<'src>>,
    pub ty: Arc<Type>,
}

impl<'src> SemCheck<InDep<'src>, ast::Program<'src>> for Program<'src> {
    async fn check0(ctx: InDep<'src>, program: ast::Program<'src>) -> anyhow::Result<Self>
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

        // プログラム全体の型は Void
        let ty = Arc::new(Primitive(Void));

        Ok(Program { span: program.span, top_elems, ty })
    }
}

impl Typed for Program<'_> {
    fn ty(&self) -> &Arc<Type> {
        &self.ty
    }
}
