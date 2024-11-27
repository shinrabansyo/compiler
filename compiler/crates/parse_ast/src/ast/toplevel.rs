use copager::ir::Tree;

use sb_compiler_parse_syntax::{SBLangDef, SBRules};

use crate::utils::unwrap_node;
use super::ConstDecl;

#[derive(Debug)]
pub enum TopLevel {
    ConstDecl(ConstDecl),
}

impl From<Tree<'_, SBLangDef>> for TopLevel  {
    fn from(tree: Tree<'_, SBLangDef>) -> Self {
        let (_, mut children) = unwrap_node(tree);
        let rhs = children.pop_front().unwrap();
        match rhs {
            // 定数宣言
            Tree::Node { tag: SBRules::ConstDecl, .. } => {
                let const_decl = ConstDecl::from(rhs);
                TopLevel::ConstDecl(const_decl)
            }
            _ => unreachable!(),
        }
    }
}
