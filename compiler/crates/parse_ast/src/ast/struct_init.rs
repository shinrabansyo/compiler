use sb_compiler_parse_cst::{Span, Spanned};

use super::{Expr, StructFieldInit, Visitor};

#[derive(Debug)]
pub struct StructInit<'src> {
    pub span: Span<'src>,
    pub ident: Span<'src>,
    pub addr: Box<Expr<'src>>,
    pub fields: Vec<StructFieldInit<'src>>,
}

impl<'src> From<Visitor<'src>> for StructInit<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        StructInit {
            span: visitor.span(),
            ident: visitor.expect_leaf().1,
            addr: Box::new(visitor.expect_node::<Expr>()),
            fields: visitor.expect_nodes::<StructFieldInit>(),
        }
    }
}

impl<'src> Spanned<'src> for StructInit<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}
