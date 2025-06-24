use sb_compiler_parse_cst::Span;
use sb_compiler_parse_syntax::SBToken;

use super::{LogicOr, Visitor};

#[derive(Debug)]
pub enum Assign<'src> {
    Normal {
        span: Span<'src>,
        ident: Span<'src>,
        assign: Box<Assign<'src>>,
    },
    Plus {
        span: Span<'src>,
        ident: Span<'src>,
        assign: Box<Assign<'src>>,
    },
    Minus {
        span: Span<'src>,
        ident: Span<'src>,
        assign: Box<Assign<'src>>,
    },
    ShiftL {
        span: Span<'src>,
        ident: Span<'src>,
        assign: Box<Assign<'src>>,
    },
    ShiftR {
        span: Span<'src>,
        ident: Span<'src>,
        assign: Box<Assign<'src>>,
    },
    ShiftRa {
        span: Span<'src>,
        ident: Span<'src>,
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
        let ident = visitor.expect_leaf().1;
        match visitor.expect_leaf().0 {
            SBToken::Assign => {
                Assign::Normal {
                    span: visitor.span(),
                    ident,
                    assign: Box::new(visitor.expect_node::<Assign>()),
                }
            }
            SBToken::PlusAssign => {
                Assign::Plus {
                    span: visitor.span(),
                    ident,
                    assign: Box::new(visitor.expect_node::<Assign>()),
                }
            }
            SBToken::MinusAssign => {
                Assign::Minus {
                    span: visitor.span(),
                    ident,
                    assign: Box::new(visitor.expect_node::<Assign>()),
                }
            }
            SBToken::ShiftLAssign => {
                Assign::ShiftL {
                    span: visitor.span(),
                    ident,
                    assign: Box::new(visitor.expect_node::<Assign>()),
                }
            }
            SBToken::ShiftRAssign => {
                Assign::ShiftR {
                    span: visitor.span(),
                    ident,
                    assign: Box::new(visitor.expect_node::<Assign>()),
                }
            }
            SBToken::ShiftRaAssign => {
                Assign::ShiftRa {
                    span: visitor.span(),
                    ident,
                    assign: Box::new(visitor.expect_node::<Assign>()),
                }
            }
            _ => unreachable!(),
        }
    }
}
