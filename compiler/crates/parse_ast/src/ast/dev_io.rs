use copager::ir::Tree;

use sb_compiler_parse_syntax::{SBLangDef, SBTokens};

use crate::utils::{unwrap_node, unwrap_leaf};
use super::Expr;

#[derive(Debug)]
pub enum DevIO {
    In {
        namespace: String,
        id: Expr,
    },
    Out {
        namespace: String,
        id: Expr,
        expr: Expr,
    }
}

impl From<(String, Tree<'_, SBLangDef>)> for DevIO  {
    fn from((namespace, tree): (String, Tree<'_, SBLangDef>)) -> Self {
        let (_, mut children) = unwrap_node(tree);

        let (tag, _) = unwrap_leaf(children.pop_front().unwrap());
        match tag {
            SBTokens::In => {
                let id = Expr::from((namespace.clone(), children.pop_front().unwrap()));
                DevIO::In { namespace, id }
            }
            SBTokens::Out => {
                let id = Expr::from((namespace.clone(), children.pop_front().unwrap()));
                let expr = Expr::from((namespace.clone(), children.pop_front().unwrap()));
                DevIO::Out { namespace, id, expr }
            }
            _ => unreachable!(),
        }
    }
}
