use copager::ir::Tree;

use sb_compiler_parse_syntax::{SBLangDef, SBTokens};

use crate::utils::{unwrap_node, unwrap_leaf};
use super::Add;

#[derive(Debug)]
pub enum BitShift {
    L {
        namespace: String,
        lhs: Box<BitShift>,
        rhs: Add,
    },
    R {
        namespace: String,
        lhs: Box<BitShift>,
        rhs: Add,
    },
    Ra {
        namespace: String,
        lhs: Box<BitShift>,
        rhs: Add,
    },
    Add {
        namespace: String,
        add: Add,
    },
}

impl From<(String, Tree<'_, SBLangDef>)> for BitShift {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);

        // 数値のみ
        if children.len() == 1 {
            let add = Add::from((namespace.clone(), children.pop_front().unwrap()));
            return BitShift::Add { namespace, add };
        }

        // 演算子付き
        let lhs = children.pop_front().unwrap();
        let op = children.pop_front().unwrap();
        let rhs = children.pop_front().unwrap();
        match unwrap_leaf(op).0 {
            SBTokens::ShiftL => {
                let lhs = Box::new(BitShift::from((namespace.clone(), lhs)));
                let rhs = Add::from((namespace.clone(), rhs));
                BitShift::L { namespace, lhs, rhs }
            }
            SBTokens::ShiftR => {
                let lhs = Box::new(BitShift::from((namespace.clone(), lhs)));
                let rhs = Add::from((namespace.clone(), rhs));
                BitShift::R { namespace, lhs, rhs }
            }
            SBTokens::ShiftRa => {
                let lhs = Box::new(BitShift::from((namespace.clone(), lhs)));
                let rhs = Add::from((namespace.clone(), rhs));
                BitShift::Ra { namespace, lhs, rhs }
            }
            _=> unreachable!(),
        }
    }
}
