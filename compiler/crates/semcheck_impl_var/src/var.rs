use std::sync::Arc;

use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_semcheck_impl_type::{Typed, Type};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Var<'src> {
    pub span: Span<'src>,
    pub ty: Arc<Type>,
}

impl<'src> Spanned<'src> for Var<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}

impl Typed for Var<'_> {
    fn ty(&self) -> Arc<Type> {
        self.ty.ty()
    }
}
