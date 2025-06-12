use std::collections::VecDeque;
use std::fmt::Debug;

use copager::cfl::token::{Token, TokenTag};
use copager::cfl::CFL;
use copager::ir::{IR, IRBuilder, RawIR};

use super::Span;

#[derive(Debug, IR, IRBuilder)]
pub enum CSTree<'input, Lang: CFL> {
    Leaf {
        tag: Lang::TokenTag,
        text: Span<'input>,
    },
    Node {
        tag: Lang::RuleTag,
        children: VecDeque<CSTree<'input, Lang>>,
    },
}

impl<'input, Lang: CFL> From<RawIR<'input, Lang>> for CSTree<'input, Lang> {
    fn from(raw: RawIR<'input, Lang>) -> Self {
        match raw {
            RawIR::Atom(token) => {
                CSTree::Leaf {
                    tag: token.kind,
                    text: Span::from(token),
                }
            },
            RawIR::List { rule: tag, elems } => {
                CSTree::Node {
                    tag,
                    children: elems.into_iter().map(CSTree::from).collect(),
                }
            }
        }
    }
}
