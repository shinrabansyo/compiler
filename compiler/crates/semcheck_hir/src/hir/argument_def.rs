use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::Span;

use super::{SemCheck, Dep};

#[derive(Debug)]
pub struct ArgumentDef<'src> {
    pub ident: Span<'src>,
    pub ty: Span<'src>,
}

impl<'src> SemCheck<Dep<'_, '_>, ast::ArgumentDef<'src>> for ArgumentDef<'src> {
    async fn check0(_: Dep<'_, '_>, arg: ast::ArgumentDef<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(ArgumentDef {
            ident: arg.ident,
            ty: arg.ty,
        })
    }
}
