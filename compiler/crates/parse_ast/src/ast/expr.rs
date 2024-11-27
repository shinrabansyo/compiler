use copager::ir::Tree;

use sb_compiler_parse_syntax::SBLangDef;

use super::Value;

#[derive(Debug)]
pub struct Expr {
    pub value: Value,
}

impl From<Tree<'_, SBLangDef>> for Expr {
    fn from(tree: Tree<'_, SBLangDef>) -> Self {
        match tree {
            Tree::Node { mut children, .. } => {
                let value = Value::from(children.pop_front().unwrap());
                Expr { value }
            }
            _ => unreachable!(),
        }
    }
}
