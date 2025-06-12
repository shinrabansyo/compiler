use sb_compiler_parse_syntax::SBToken;

use super::{LogicOr, Visitor};

#[derive(Debug)]
pub enum Assign {
    Normal {
        ident: String,
        assign: Box<Assign>,
    },
    Plus {
        ident: String,
        assign: Box<Assign>,
    },
    Minus {
        ident: String,
        assign: Box<Assign>,
    },
    ShiftL {
        ident: String,
        assign: Box<Assign>,
    },
    ShiftR {
        ident: String,
        assign: Box<Assign>,
    },
    ShiftRa {
        ident: String,
        assign: Box<Assign>,
    },
    LogicOr {
        or: LogicOr,
    }
}

impl From<Visitor<'_>> for Assign {
    fn from(mut visitor: Visitor<'_>) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return Assign::LogicOr {
                or: visitor.expect_node::<LogicOr>(),
            };
        }

        // 演算子付き
        let ident = visitor.expect_leaf().1.to_string();
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
