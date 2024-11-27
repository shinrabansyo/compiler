use copager::ir::Tree;

use sb_compiler_parse_syntax::SBLangDef;

use crate::utils::unwrap_node;
use super::TopLevel;

#[derive(Debug)]
pub struct Program {
    pub top_level_elems: Vec<TopLevel>,
}

impl From<Tree<'_, SBLangDef>> for Program {
    fn from(tree: Tree<'_, SBLangDef>) -> Self {
        Program {
            top_level_elems: expand_lrec(tree),
        }
    }
}

fn expand_lrec(tree: Tree<'_, SBLangDef>) -> Vec<TopLevel> {
    let (_, mut children) = unwrap_node(tree);
    if children.len() == 1 {
        vec![TopLevel::from((
            "global".to_string(),
            children.pop_front().unwrap()
        ))]
    } else {
        let mut top_level_elems = expand_lrec(children.pop_front().unwrap());
        top_level_elems.push(TopLevel::from((
            "global".to_string(),
            children.pop_front().unwrap()
        )));
        top_level_elems
    }
}
