use copager::ir::Tree;

use sb_compiler_parse_syntax::SBLangDef;

use crate::utils::unwrap_node;
use super::Cond;

#[derive(Debug)]
pub enum LogicAnd {
    And {
        namespace: String,
        lhs: Box<LogicAnd>,
        rhs: Cond,
    },
    Cond {
        namespace: String,
        cond: Cond,
    },
}

impl From<(String, Tree<'_, SBLangDef>)> for LogicAnd {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);

        // 数値のみ
        if children.len() == 1 {
            let cond = Cond::from((namespace.clone(), children.pop_front().unwrap()));
            return LogicAnd::Cond { namespace, cond };
        }

        // 演算子付き
        let lhs = children.pop_front().unwrap();
        let lhs = Box::new(LogicAnd::from((namespace.clone(), lhs)));

        let _op = children.pop_front().unwrap();

        let rhs = children.pop_front().unwrap();
        let rhs = Cond::from((namespace.clone(), rhs));

        LogicAnd::And { namespace, lhs, rhs }
    }
}
