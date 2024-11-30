use copager::ir::Tree;

use sb_compiler_parse_syntax::{SBLangDef, SBTokens};

use crate::utils::{unwrap_node, unwrap_leaf};
use super::Add;

#[derive(Debug)]
pub enum Cond {
    Eq {
        namespace: String,
        lhs: Box<Cond>,
        rhs: Add,
    },
    Neq {
        namespace: String,
        lhs: Box<Cond>,
        rhs: Add,
    },
    Lt {
        namespace: String,
        lhs: Box<Cond>,
        rhs: Add,
    },
    Lte {
        namespace: String,
        lhs: Box<Cond>,
        rhs: Add,
    },
    Gt {
        namespace: String,
        lhs: Box<Cond>,
        rhs: Add,
    },
    Gte {
        namespace: String,
        lhs: Box<Cond>,
        rhs: Add,
    },
    Add {
        namespace: String,
        add: Add,
    },
}

impl From<(String, Tree<'_, SBLangDef>)> for Cond {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);

        // 数値のみ
        if children.len() == 1 {
            let add = Add::from((namespace.clone(), children.pop_front().unwrap()));
            return Cond::Add { namespace, add };
        }

        // 演算子付き
        let lhs = children.pop_front().unwrap();
        let op = children.pop_front().unwrap();
        let rhs = children.pop_front().unwrap();
        match unwrap_leaf(op).0 {
            SBTokens::Eq => {
                let lhs = Box::new(Cond::from((namespace.clone(), lhs)));
                let rhs = Add::from((namespace.clone(), rhs));
                Cond::Eq { namespace, lhs, rhs }
            }
            SBTokens::Neq => {
                let lhs = Box::new(Cond::from((namespace.clone(), lhs)));
                let rhs = Add::from((namespace.clone(), rhs));
                Cond::Neq { namespace, lhs, rhs }
            }
            SBTokens::Lt => {
                let lhs = Box::new(Cond::from((namespace.clone(), lhs)));
                let rhs = Add::from((namespace.clone(), rhs));
                Cond::Lt { namespace, lhs, rhs }
            }
            SBTokens::Lte => {
                let lhs = Box::new(Cond::from((namespace.clone(), lhs)));
                let rhs = Add::from((namespace.clone(), rhs));
                Cond::Lte { namespace, lhs, rhs }
            }
            SBTokens::Gt => {
                let lhs = Box::new(Cond::from((namespace.clone(), lhs)));
                let rhs = Add::from((namespace.clone(), rhs));
                Cond::Gt { namespace, lhs, rhs }
            }
            SBTokens::Gte => {
                let lhs = Box::new(Cond::from((namespace.clone(), lhs)));
                let rhs = Add::from((namespace.clone(), rhs));
                Cond::Gte { namespace, lhs, rhs }
            }
            _=> unreachable!(),
        }
    }
}
