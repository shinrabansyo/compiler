use sb_compiler_parse_cst::{Span, Spanned};

use super::{ValueR, Visitor};

#[derive(Debug)]
pub struct StructAccess<'src> {
    pub span: Span<'src>,
    pub lhs: Box<ValueR<'src>>,
    pub rhs: Span<'src>,
}

impl<'src> From<Visitor<'src>> for StructAccess<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        // 演算子付き
        let span = visitor.span();
        let lhs = Box::new(visitor.expect_node::<ValueR>());
        let _ = visitor.expect_leaf();
        let rhs = visitor.expect_leaf().1;

        StructAccess { span, lhs, rhs }
    }
}

impl<'src> Spanned<'src> for StructAccess<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}
