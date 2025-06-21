use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::Span;

use super::{SemCheckFrom, Dep};

#[derive(Debug)]
pub struct ArgumentDef<'input> {
    pub ident: Span<'input>,
    pub ty: Span<'input>,
}

impl<'input> SemCheckFrom<Dep<'_>, ast::ArgumentDef<'input>> for ArgumentDef<'input> {
    async fn check0(_: Dep<'_>, arg: ast::ArgumentDef<'input>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(ArgumentDef {
            ident: arg.ident,
            ty: arg.ty,
        })
    }
}
