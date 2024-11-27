use copager::ir::Tree;

use sb_compiler_parse_syntax::{SBLangDef, SBRules};

use crate::utils::unwrap_node;
use super::ConstDecl;

#[derive(Debug)]
pub enum Top {
    ConstDecl {
        namespace: String,
        const_decl: ConstDecl,
    },
}

impl From<(String, Tree<'_, SBLangDef>)> for Top {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);
        let rhs = children.pop_front().unwrap();
        match rhs {
            // 定数宣言
            Tree::Node { tag: SBRules::ConstDecl, .. } => {
                let const_decl = ConstDecl::from((namespace.clone(), rhs));
                Top::ConstDecl { namespace, const_decl }
            }
            _ => unreachable!(),
        }
    }
}
