use copager::ir::Tree;

use sb_compiler_parse_syntax::SBLangDef;

use crate::utils::{unwrap_node, unwrap_leaf};
use super::LogicOr;

#[derive(Debug)]
pub enum Assign {
    Assign {
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
        let assign = Box::new(Assign::from((namespace.clone(), children.pop_front().unwrap())));
        Assign::Assign { namespace, ident, assign }
    }
}
