use sb_compiler_parse_cst::Span;

use super::{Unary, Visitor};

#[derive(Debug)]
pub enum Cast<'src> {
    Casting {
        span: Span<'src>,
        unary: Unary<'src>,
        ty: Span<'src>,
    },
    Unary {
        unary: Unary<'src>
    },
}

impl<'src> From<Visitor<'src>> for Cast<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return Cast::Unary {
                unary: visitor.expect_node::<Unary>(),
            };
        }

        // キャスト指示付き
        let span = visitor.span();
        let unary = visitor.expect_node::<Unary>();
        let ty = visitor.expect_leaf().1;

        Cast::Casting { span, unary, ty }
    }
}
