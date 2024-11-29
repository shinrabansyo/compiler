use copager::ir::Tree;

use sb_compiler_parse_syntax::SBLangDef;

use crate::utils::expand_lrec;
use super::Top;

#[derive(Debug)]
pub struct Program {
    pub top_elems: Vec<Top>,
}

impl From<Tree<'_, SBLangDef>> for Program {
    fn from(tree: Tree<'_, SBLangDef>) -> Self {
        Program {
            top_elems: expand_lrec::<Top>("global".to_string(), tree),
        }
    }
}
