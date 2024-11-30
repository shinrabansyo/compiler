use copager::ir::Tree;

use sb_compiler_parse_syntax::{SBLangDef, SBTokens};

use crate::utils::{unwrap_node, unwrap_leaf};
use super::LogicOr;

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

impl From<(String, Tree<'_, SBLangDef>)> for Assign {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);

        // 数値のみ
        if children.len() == 1 {
            let or = LogicOr::from((namespace.clone(), children.pop_front().unwrap()));
            return Assign::LogicOr { namespace, or };
        }

        // 演算子付き
        let (_, ident) = unwrap_leaf(children.pop_front().unwrap());
        let ident = ident.to_string();
        let op = children.pop_front().unwrap();
        let rhs = children.pop_front().unwrap();
        match unwrap_leaf(op).0 {
            SBTokens::Assign => {
                let assign = Box::new(Assign::from((namespace.clone(), rhs)));
                Assign::Normal { namespace, ident, assign }
            }
            SBTokens::PlusAssign => {
                let assign = Box::new(Assign::from((namespace.clone(), rhs)));
                Assign::Plus { namespace, ident, assign }
            }
            SBTokens::MinusAssign => {
                let assign = Box::new(Assign::from((namespace.clone(), rhs)));
                Assign::Minus { namespace, ident, assign }
            }
            SBTokens::ShiftLAssign => {
                let assign = Box::new(Assign::from((namespace.clone(), rhs)));
                Assign::ShiftL { namespace, ident, assign }
            }
            SBTokens::ShiftRAssign => {
                let assign = Box::new(Assign::from((namespace.clone(), rhs)));
                Assign::ShiftR { namespace, ident, assign }
            }
            SBTokens::ShiftRaAssign => {
                let assign = Box::new(Assign::from((namespace.clone(), rhs)));
                Assign::ShiftRa { namespace, ident, assign }
            }
            _ => unreachable!(),
        }
    }
}
