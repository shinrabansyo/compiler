use sb_compiler_parse_syntax::SBTokens;

use super::{LogicOr, Visitor};

#[derive(Debug)]
pub enum Assign {
    Normal {
        namespace: String,
        ident: String,
        assign: Box<Assign>,
    },
    Plus {
        namespace: String,
        ident: String,
        assign: Box<Assign>,
    },
    Minus {
        namespace: String,
        ident: String,
        assign: Box<Assign>,
    },
    ShiftL {
        namespace: String,
        ident: String,
        assign: Box<Assign>,
    },
    ShiftR {
        namespace: String,
        ident: String,
        assign: Box<Assign>,
    },
    ShiftRa {
        namespace: String,
        ident: String,
        assign: Box<Assign>,
    },
    LogicOr {
        namespace: String,
        or: LogicOr,
    }
}

impl From<(String, Visitor<'_>)> for Assign {
    fn from((namespace, mut visitor): (String, Visitor<'_>)) -> Self {
        // 数値のみ
        if visitor.len() == 1 {
            return Assign::LogicOr {
                namespace,
                or: visitor.expect_node::<LogicOr>(),
            };
        }

        // 演算子付き
        let ident = visitor.expect_leaf().1.to_string();
        match visitor.expect_leaf().0 {
            SBTokens::Assign => {
                Assign::Normal {
                    namespace,
                    ident,
                    assign: Box::new(visitor.expect_node::<Assign>()),
                }
            }
            SBTokens::PlusAssign => {
                Assign::Plus {
                    namespace,
                    ident,
                    assign: Box::new(visitor.expect_node::<Assign>()),
                }
            }
            SBTokens::MinusAssign => {
                Assign::Minus {
                    namespace,
                    ident,
                    assign: Box::new(visitor.expect_node::<Assign>()),
                }
            }
            SBTokens::ShiftLAssign => {
                Assign::ShiftL {
                    namespace,
                    ident,
                    assign: Box::new(visitor.expect_node::<Assign>()),
                }
            }
            SBTokens::ShiftRAssign => {
                Assign::ShiftR {
                    namespace,
                    ident,
                    assign: Box::new(visitor.expect_node::<Assign>()),
                }
            }
            SBTokens::ShiftRaAssign => {
                Assign::ShiftRa {
                    namespace,
                    ident,
                    assign: Box::new(visitor.expect_node::<Assign>()),
                }
            }
            _ => unreachable!(),
        }
    }
}
