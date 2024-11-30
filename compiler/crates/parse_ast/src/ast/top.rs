use copager::ir::Tree;

use sb_compiler_parse_syntax::{SBLangDef, SBRules};

use crate::utils::unwrap_node;
use super::{FuncDef, VarDecl};

#[derive(Debug)]
pub enum Top {
    FuncDef {
        namespace: String,
        func_def: FuncDef,
    },
    VarDecl {
        namespace: String,
        var_decl: VarDecl,
    },
}

impl From<(String, Tree<'_, SBLangDef>)> for Top {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);
        let rhs = children.pop_front().unwrap();
        match rhs {
            Tree::Node { tag: SBRules::FuncDef, .. } => {
                let func_def = FuncDef::from((namespace.clone(), rhs));
                Top::FuncDef { namespace, func_def }
            }
            Tree::Node { tag: SBRules::VarDecl, .. } => {
                let var_decl = VarDecl::from((namespace.clone(), rhs));
                Top::VarDecl { namespace, var_decl }
            }
            _ => unreachable!(),
        }
    }
}
