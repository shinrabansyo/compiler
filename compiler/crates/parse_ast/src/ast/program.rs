use copager::ir::Tree;

use sb_compiler_parse_syntax::SBLangDef;

use crate::utils::unwrap_node;
use super::Top;

#[derive(Debug)]
pub struct Program {
    pub top_elems: Vec<Top>,
}

impl From<Tree<'_, SBLangDef>> for Program {
    fn from(tree: Tree<'_, SBLangDef>) -> Self {
        Program {
            top_elems: expand_lrec(tree),
        }
    }
}

fn expand_lrec(tree: Tree<'_, SBLangDef>) -> Vec<Top> {
    let (_, mut children) = unwrap_node(tree);
    if children.len() == 1 {
        vec![Top::from((
            "global".to_string(),
            children.pop_front().unwrap()
        ))]
    } else {
        let mut top_elems = expand_lrec(children.pop_front().unwrap());
        top_elems.push(Top::from((
            "global".to_string(),
            children.pop_front().unwrap()
        )));
        top_elems
    }
}
