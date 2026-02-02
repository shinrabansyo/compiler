use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_parse_syntax::SBToken;

use super::{LogicOr, ValueL, Visitor};

#[derive(Debug)]
pub enum Assign<'src> {
    Normal {
        span: Span<'src>,
        lhs: ValueL<'src>,
        assign: Box<Assign<'src>>,
    },
    Plus {
        span: Span<'src>,
        lhs: ValueL<'src>,
        assign: Box<Assign<'src>>,
    },
    Minus {
        span: Span<'src>,
        lhs: ValueL<'src>,
        assign: Box<Assign<'src>>,
    },
    Mul {
        span: Span<'src>,
        lhs: ValueL<'src>,
        assign: Box<Assign<'src>>,
    },
    Div {
        span: Span<'src>,
        lhs: ValueL<'src>,
        assign: Box<Assign<'src>>,
    },
    Mod {
        span: Span<'src>,
        lhs: ValueL<'src>,
        assign: Box<Assign<'src>>,
    },
    ShiftL {
        span: Span<'src>,
        lhs: ValueL<'src>,
        assign: Box<Assign<'src>>,
    },
    ShiftR {
        span: Span<'src>,
        lhs: ValueL<'src>,
        assign: Box<Assign<'src>>,
    },
    ShiftRa {
        span: Span<'src>,
        lhs: ValueL<'src>,
        assign: Box<Assign<'src>>,
    },
    LogicOr {
        or: LogicOr<'src>,
    }
}

impl<'src> From<Visitor<'src>> for Assign<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return Assign::LogicOr {
                or: visitor.expect_node::<LogicOr>(),
            };
        }

        // 演算子付き
        let lhs = visitor.expect_node::<ValueL>();
        match visitor.expect_leaf().0 {
            SBToken::Assign => {
                Assign::Normal {
                    span: visitor.span(),
                    lhs,
                    assign: Box::new(visitor.expect_node::<Assign>()),
                }
            }
            SBToken::PlusAssign => {
                Assign::Plus {
                    span: visitor.span(),
                    lhs,
                    assign: Box::new(visitor.expect_node::<Assign>()),
                }
            }
            SBToken::MinusAssign => {
                Assign::Minus {
                    span: visitor.span(),
                    lhs,
                    assign: Box::new(visitor.expect_node::<Assign>()),
                }
            }
            SBToken::MulAssign => {
                Assign::Mul {
                    span: visitor.span(),
                    lhs,
                    assign: Box::new(visitor.expect_node::<Assign>()),
                }
            }
            SBToken::DivAssign => {
                Assign::Div {
                    span: visitor.span(),
                    lhs,
                    assign: Box::new(visitor.expect_node::<Assign>()),
                }
            }
            SBToken::ModAssign => {
                Assign::Mod {
                    span: visitor.span(),
                    lhs,
                    assign: Box::new(visitor.expect_node::<Assign>()),
                }
            }
            SBToken::ShiftLAssign => {
                Assign::ShiftL {
                    span: visitor.span(),
                    lhs,
                    assign: Box::new(visitor.expect_node::<Assign>()),
                }
            }
            SBToken::ShiftRAssign => {
                Assign::ShiftR {
                    span: visitor.span(),
                    lhs,
                    assign: Box::new(visitor.expect_node::<Assign>()),
                }
            }
            SBToken::ShiftRaAssign => {
                Assign::ShiftRa {
                    span: visitor.span(),
                    lhs,
                    assign: Box::new(visitor.expect_node::<Assign>()),
                }
            }
            _ => unreachable!(),
        }
    }
}

impl<'src> Spanned<'src> for Assign<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            Assign::Normal { span, .. } => *span,
            Assign::Plus { span, .. } => *span,
            Assign::Minus { span, .. } => *span,
            Assign::Mul { span, .. } => *span,
            Assign::Div { span, .. } => *span,
            Assign::Mod { span, .. } => *span,
            Assign::ShiftL { span, .. } => *span,
            Assign::ShiftR { span, .. } => *span,
            Assign::ShiftRa { span, .. } => *span,
            Assign::LogicOr { or } => or.span(),
        }
    }
}
