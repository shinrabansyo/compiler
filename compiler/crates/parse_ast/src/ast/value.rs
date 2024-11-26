use copager::ir::Tree;

use sb_compiler_parse_syntax::{SBLangDef, SBTokens};

#[derive(Debug)]
pub enum Value {
    Const(i32),
}

impl From<Tree<'_, SBLangDef>> for Value {
    fn from(tree: Tree<'_, SBLangDef>) -> Self {
        match tree {
            Tree::Leaf { tag: SBTokens::Num, text } => {
                let value = text.parse().unwrap();
                Value::Const(value)
            }
            _ => unreachable!(),
        }
    }
}
