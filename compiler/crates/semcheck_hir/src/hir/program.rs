use sb_compiler_parse_ast as ast;
use sb_compiler_semcheck_async::prelude::*;
use sb_compiler_semcheck_impl_type_decl::r#type::primitive::*;
use sb_compiler_semcheck_impl_type_decl::r#type::*;
use sb_compiler_utils::error::ErrComposer;

use super::{Top, SemCheck, InDep};

#[derive(Debug)]
pub struct Program<'src> {
    pub top_elems: Vec<Top<'src>>,
}

impl<'src> SemCheck<InDep<'src>, ast::Program<'src>> for Program<'src> {
    async fn check0(ctx: InDep<'src>, program: ast::Program<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let top_elems = program
            .top_elems
            .into_iter()
            .map(|top| Top::check(ctx.clone(), top))
            .join_all()
            .await
            .compose()?;

        Ok(Program { top_elems })
    }

    fn ty(&self) -> &Type {
        &Primitive(Void)
    }
}
