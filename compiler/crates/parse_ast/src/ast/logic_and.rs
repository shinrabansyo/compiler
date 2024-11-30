use copager::ir::Tree;

use sb_compiler_parse_syntax::SBLangDef;

use crate::utils::unwrap_node;
use super::BitOr;

#[derive(Debug)]
pub enum LogicAnd {
    And {
        namespace: String,
        lhs: Box<LogicAnd>,
        rhs: BitOr,
    },
    BitOr {
        namespace: String,
        or: BitOr,
    },
}

impl From<(String, Tree<'_, SBLangDef>)> for LogicAnd {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);

        // 数値のみ
        if children.len() == 1 {
            let or = BitOr::from((namespace.clone(), children.pop_front().unwrap()));
            return LogicAnd::BitOr { namespace, or };
        }

        // 演算子付き
        let lhs = children.pop_front().unwrap();
        let lhs = Box::new(LogicAnd::from((namespace.clone(), lhs)));

        let _op = children.pop_front().unwrap();

        let rhs = children.pop_front().unwrap();
        let rhs = BitOr::from((namespace.clone(), rhs));

        LogicAnd::And { namespace, lhs, rhs }
    }
}
