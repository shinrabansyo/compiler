use sb_compiler_parse_syntax::SBToken;

use super::{LogicOr, Visitor};

#[derive(Debug)]
pub enum Assign<'input> {
    Normal {
        ident: &'input str,
        assign: Box<Assign<'input>>,
    },
    Plus {
        ident: &'input str,
        assign: Box<Assign<'input>>,
    },
    Minus {
        ident: &'input str,
        assign: Box<Assign<'input>>,
    },
    ShiftL {
        ident: &'input str,
        assign: Box<Assign<'input>>,
    },
    ShiftR {
        ident: &'input str,
        assign: Box<Assign<'input>>,
    },
    ShiftRa {
        ident: &'input str,
        assign: Box<Assign<'input>>,
    },
    LogicOr {
        or: LogicOr<'input>,
    }
}

impl<'input> From<Visitor<'input>> for Assign<'input> {
    fn from(mut visitor: Visitor<'input>) -> Self {
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
