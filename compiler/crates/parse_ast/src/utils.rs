use std::collections::VecDeque;

use copager::ir::Tree;

use sb_compiler_parse_syntax::{SBLangDef, SBRules, SBTokens};

pub(crate) fn unwrap_node(node: Tree<'_, SBLangDef>) -> (SBRules, VecDeque<Tree<'_, SBLangDef>>) {
    match node {
        Tree::Node { tag, children } => (tag, children),
        _ => unreachable!(),
    }
}

pub(crate) fn unwrap_leaf(leaf: Tree<'_, SBLangDef>) -> (SBTokens, &str) {
    match leaf {
        Tree::Leaf { tag, text } => (tag, text),
        _ => unreachable!(),
    }
}

pub(crate) fn expand_lrec<'a, T>(namespace: String, tree: Tree<'a, SBLangDef>) -> Vec<T>
where
    T: From<(String, Tree<'a, SBLangDef>)>,
{
    let (_, mut children) = unwrap_node(tree);
    match children.len() {
        0 => vec![],
        1 => vec![T::from((namespace, children.pop_front().unwrap()))],
        _ => {
            let mut elems = expand_lrec(namespace.clone(),children.pop_front().unwrap());
            elems.push(T::from((namespace, children.pop_front().unwrap())));
            elems
        }
    }
}
