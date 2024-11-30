use copager::ir::Tree;

use sb_compiler_parse_syntax::{SBLangDef, SBRules};

use crate::utils::unwrap_node;
use super::{VarDecl, Block, Expr};

#[derive(Debug)]
pub enum Stmt {
    VarDecl {
        namespace: String,
        var_decl: VarDecl,
    },
    Block {
        namespace: String,
        block: Block,
    },
    Expr {
        namespace: String,
        expr: Expr,
    },
    Return {
        namespace: String,
        expr: Expr,
    }
}

impl From<(String, Tree<'_, SBLangDef>)> for Stmt {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);
        let rhs = children.pop_front().unwrap();
        match rhs {
            Tree::Node { tag: SBRules::VarDecl, .. } => {
                let var_decl = VarDecl::from((namespace.clone(), rhs));
                Stmt::VarDecl { namespace, var_decl }
            }
            Tree::Node { tag: SBRules::Block, .. } => {
                let block = Block::from((namespace.clone(), rhs));
                Stmt::Block { namespace, block }
            }
            Tree::Node { tag: SBRules::Expr, .. } => {
                let expr = Expr::from((namespace.clone(), rhs));
                Stmt::Expr { namespace, expr }
            }
            Tree::Node { tag: SBRules::Return, mut children } => {
                let expr = Expr::from((namespace.clone(), children.pop_front().unwrap()));
                Stmt::Return { namespace, expr }
            }
            _ => unreachable!(),
        }
    }
}
