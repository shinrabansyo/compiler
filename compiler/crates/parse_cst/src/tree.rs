use std::collections::VecDeque;
use std::fmt::Debug;

use copager::cfl::token::Token;
use copager::cfl::CFL;
use copager::ir::{IR, IRBuilder};
use copager::prelude::*;

use super::Span;

#[derive(Debug, IR)]
pub enum CSTree<'input, Lang: CFL> {
    Leaf {
        span: Span<'input>,
        tag: Lang::TokenTag,
        token: Token<'input, Lang::TokenTag>,
    },
    Node {
        span: Span<'input>,
        tag: Lang::RuleTag,
        children: VecDeque<CSTree<'input, Lang>>,
    },
}

impl<'input, Lang: CFL> CSTree<'input, Lang> {
    pub fn span(&self) -> Span<'input> {
        match self {
            CSTree::Leaf { span, .. } => *span,
            CSTree::Node { span, .. } => *span,
        }
    }
}

pub struct CSTreeBuilder<'input, Lang: CFL> {
    stack: VecDeque<CSTree<'input, Lang>>,
}

impl<'input, Lang: CFL> IRBuilder<'input, Lang> for CSTreeBuilder<'input, Lang> {
    type Output = CSTree<'input, Lang>;

    fn new() -> Self {
        CSTreeBuilder {
            stack: VecDeque::new(),
        }
    }

    fn on_read(&mut self, token: Token<'input, Lang::TokenTag>) -> anyhow::Result<()> {
        let tree = CSTree::Leaf {
            span: Span::from(token),
            tag: token.kind,
            token,
        };
        self.stack.push_back(tree);
        Ok(())
    }

    fn on_parse(&mut self, tag: Lang::RuleTag, len: usize) -> anyhow::Result<()> {
        // Node 全体の Span の計算
        let children = self.stack.split_off(self.stack.len() - len);
        let span = match children.len() {
            0 => {
                // スタック端の Span を基に計算
                let span_stack_last = match self.stack.back() {
                    Some(tree) => tree.span(),
                    None => Span {
                        src: "",
                        body: (0, 0),
                        full: (0, 0),
                    },
                };
                Span {
                    src: span_stack_last.src,
                    body: (span_stack_last.body.1, span_stack_last.body.1),
                    full: (span_stack_last.full.1, span_stack_last.full.1),
                }
            }
            _ => {
                // 子要素の両端の Span を基に計算
                let span_first = children.front().unwrap().span();
                let span_last = children.back().unwrap().span();
                Span {
                    src: span_first.src,
                    body: (span_first.body.0, span_last.body.1),
                    full: (span_first.full.0, span_last.full.1),
                }
            }
        };

        // Node を生成してスタックに追加
        let children = children
            .into_iter()
            .filter(|elem| match elem {
                CSTree::Leaf {token, .. } => {
                    !token.kind.as_option_list().contains(&"ir_omit")
                }
                _ => true,
            })
            .collect();
        let tree = CSTree::Node { span, tag, children };
        self.stack.push_back(tree);

        Ok(())
    }

    fn build(mut self) -> anyhow::Result<Self::Output> {
        Ok(self.stack.pop_back().unwrap())
    }
}
