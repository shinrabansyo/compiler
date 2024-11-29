use copager::ir::Tree;

use sb_compiler_parse_syntax::{SBLangDef, SBRules};

use crate::utils::unwrap_node;
use super::{ConstDecl, Block, Expr};

#[derive(Debug)]
pub enum Stmt {
    ConstDecl {
        namespace: String,
        const_decl: ConstDecl,
    },
    Block {
        namespace: String,
        block: Block,
    },
    Expr {
        namespace: String,
        expr: Expr,
    }
}

impl From<(String, Tree<'_, SBLangDef>)> for Stmt {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);
        let rhs = children.pop_front().unwrap();
        match rhs {
            Tree::Node { tag: SBRules::ConstDecl, .. } => {
                let const_decl = ConstDecl::from((namespace.clone(), rhs));
                Stmt::ConstDecl { namespace, const_decl }
            }
            Tree::Node { tag: SBRules::Block, .. } => {
                let block = Block::from((namespace.clone(), rhs));
                Stmt::Block { namespace, block }
            }
            Tree::Node { tag: SBRules::Expr, .. } => {
                let expr = Expr::from((namespace.clone(), rhs));
                Stmt::Expr { namespace, expr }
            }
            _ => unreachable!(),
        }
    }
}
