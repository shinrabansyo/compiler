use copager::ir::Tree;

use sb_compiler_parse_syntax::SBLangDef;

use crate::utils::unwrap_node;
use super::LogicAnd;

#[derive(Debug)]
pub enum LogicOr {
    Or {
        namespace: String,
        lhs: Box<LogicOr>,
        rhs: LogicAnd,
    },
    LogicAnd {
        namespace: String,
        and: LogicAnd,
    },
}

impl From<(String, Tree<'_, SBLangDef>)> for LogicOr {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);

        // 数値のみ
        if children.len() == 1 {
            let and = LogicAnd::from((namespace.clone(), children.pop_front().unwrap()));
            return LogicOr::LogicAnd { namespace, and };
        }

        // 演算子付き
        let lhs = children.pop_front().unwrap();
        let lhs = Box::new(LogicOr::from((namespace.clone(), lhs)));

        let _op = children.pop_front().unwrap();

        let rhs = children.pop_front().unwrap();
        let rhs = LogicAnd::from((namespace.clone(), rhs));

        LogicOr::Or { namespace, lhs, rhs }
    }
}
