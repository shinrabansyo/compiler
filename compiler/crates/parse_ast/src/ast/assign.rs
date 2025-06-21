use sb_compiler_parse_cst::Span;
use sb_compiler_parse_syntax::SBToken;

use super::{LogicOr, Visitor};

#[derive(Debug)]
pub enum Assign<'src> {
    Normal {
        ident: Span<'src>,
        assign: Box<Assign<'src>>,
    },
    Plus {
        ident: Span<'src>,
        assign: Box<Assign<'src>>,
    },
    Minus {
        ident: Span<'src>,
        assign: Box<Assign<'src>>,
    },
    ShiftL {
        ident: Span<'src>,
        assign: Box<Assign<'src>>,
    },
    ShiftR {
        ident: Span<'src>,
        assign: Box<Assign<'src>>,
    },
    ShiftRa {
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
                    ident,
                    assign: Box::new(visitor.expect_node::<Assign>()),
                }
            }
            SBToken::PlusAssign => {
                Assign::Plus {
                    ident,
                    assign: Box::new(visitor.expect_node::<Assign>()),
                }
            }
            SBToken::MinusAssign => {
                Assign::Minus {
                    ident,
                    assign: Box::new(visitor.expect_node::<Assign>()),
                }
            }
            SBToken::ShiftLAssign => {
                Assign::ShiftL {
                    ident,
                    assign: Box::new(visitor.expect_node::<Assign>()),
                }
            }
            SBToken::ShiftRAssign => {
                Assign::ShiftR {
                    ident,
                    assign: Box::new(visitor.expect_node::<Assign>()),
                }
            }
            SBToken::ShiftRaAssign => {
                Assign::ShiftRa {
                    ident,
                    assign: Box::new(visitor.expect_node::<Assign>()),
                }
            }
            _ => unreachable!(),
        }
    }
}
