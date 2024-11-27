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
