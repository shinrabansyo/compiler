use copager::ir::Tree;

use sb_compiler_parse_syntax::{SBLangDef, SBTokens};

use crate::utils::{unwrap_node, unwrap_leaf};
use super::Add;

#[derive(Debug)]
pub enum Logic {
    Eq {
        namespace: String,
        lhs: Box<Logic>,
        rhs: Add,
    },
    Neq {
        namespace: String,
        lhs: Box<Logic>,
        rhs: Add,
    },
    Lt {
        namespace: String,
        lhs: Box<Logic>,
        rhs: Add,
    },
    Lte {
        namespace: String,
        lhs: Box<Logic>,
        rhs: Add,
    },
    Gt {
        namespace: String,
        lhs: Box<Logic>,
        rhs: Add,
    },
    Gte {
        namespace: String,
        lhs: Box<Logic>,
        rhs: Add,
    },
    Add {
        namespace: String,
        add: Add,
    },
}

impl From<(String, Tree<'_, SBLangDef>)> for Logic {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);

        // 数値のみ
        if children.len() == 1 {
            let add = Add::from((namespace.clone(), children.pop_front().unwrap()));
            return Logic::Add { namespace, add };
        }

        // 演算子付き
        let lhs = children.pop_front().unwrap();
        let op = children.pop_front().unwrap();
        let rhs = children.pop_front().unwrap();
        match unwrap_leaf(op).0 {
            SBTokens::Eq => {
                let lhs = Box::new(Logic::from((namespace.clone(), lhs)));
                let rhs = Add::from((namespace.clone(), rhs));
                Logic::Eq { namespace, lhs, rhs }
            }
            SBTokens::Neq => {
                let lhs = Box::new(Logic::from((namespace.clone(), lhs)));
                let rhs = Add::from((namespace.clone(), rhs));
                Logic::Neq { namespace, lhs, rhs }
            }
            SBTokens::Lt => {
                let lhs = Box::new(Logic::from((namespace.clone(), lhs)));
                let rhs = Add::from((namespace.clone(), rhs));
                Logic::Lt { namespace, lhs, rhs }
            }
            SBTokens::Lte => {
                let lhs = Box::new(Logic::from((namespace.clone(), lhs)));
                let rhs = Add::from((namespace.clone(), rhs));
                Logic::Lte { namespace, lhs, rhs }
            }
            SBTokens::Gt => {
                let lhs = Box::new(Logic::from((namespace.clone(), lhs)));
                let rhs = Add::from((namespace.clone(), rhs));
                Logic::Gt { namespace, lhs, rhs }
            }
            SBTokens::Gte => {
                let lhs = Box::new(Logic::from((namespace.clone(), lhs)));
                let rhs = Add::from((namespace.clone(), rhs));
                Logic::Gte { namespace, lhs, rhs }
            }
            _=> unreachable!(),
        }
    }
}
