use copager::cfl::token::{Token, TokenTag};
use copager::cfl::CFL;
use copager::ir::{IR, IRBuilder, RawIR, Tree};

#[derive(Debug, IR, IRBuilder)]
pub struct CSTreeVisitor<'input, Lang: CFL> {
    cst: Option<Tree<'input, Lang>>,
}

impl <'input, Lang: CFL> From<RawIR<'input, Lang>> for CSTreeVisitor<'input, Lang> {
    fn from(raw_ir: RawIR<'input, Lang>) -> Self {
        CSTreeVisitor {
            cst: Some(Tree::from(raw_ir)),
        }
    }
}

impl<'input, Lang: CFL> CSTreeVisitor<'input, Lang> {
    pub fn len(&self) -> usize {
        match &self.cst {
            Some(Tree::Node { children, .. }) => children.len(),
            Some(Tree::Leaf { .. }) => 1,
            None => 0,
        }
    }

    pub fn peek(&self) -> (Option<Lang::TokenTag>, Option<Lang::RuleTag>) {
        match &self.cst {
            Some(Tree::Leaf { tag, .. }) => (Some(*tag), None),
            Some(Tree::Node { children, .. }) => {
                match children.get(0) {
                    Some(Tree::Leaf { tag, .. }) => (Some(*tag), None),
                    Some(Tree::Node { tag, .. }) => (None, Some(*tag)),
                    None => (None, None),
                }
            },
            None => (None, None),
        }
    }

    pub fn expect_leaf(&mut self) -> (Lang::TokenTag, &'input str) {
        match self.pop_front() {
            Some(Tree::Leaf { tag, text }) => (tag, text),
            Some(..) => panic!("Expected a leaf but found a node"),
            None => panic!("No more elements in the CSTreeVisitor"),
        }
    }

    pub fn expect_node<T>(&mut self) -> T
    where
        T: From<CSTreeVisitor<'input, Lang>>,
    {
        match self.pop_spawn() {
            Some(node_visitor) => T::from(node_visitor),
            None => panic!("No more elements in the CSTreeVisitor"),
        }
    }

    pub fn expect_nodes<T>(&mut self) -> Vec<T>
    where
        T: From<CSTreeVisitor<'input, Lang>>,
    {
        match self.pop_spawn() {
            Some(mut node_visitor) => node_visitor.expect_nodes_lrec::<T>(),
            None => vec![],
        }
    }

    fn expect_nodes_lrec<T>(&mut self) -> Vec<T>
    where
        T: From<CSTreeVisitor<'input, Lang>>,
    {
        match (self.pop_spawn(), self.pop_spawn()) {
            (Some(mut lrec_visitor), Some(last_visitor)) => {
                let mut lrec_elems = lrec_visitor.expect_nodes_lrec::<T>();
                let last_elem = T::from(last_visitor);
                lrec_elems.push(last_elem);
                lrec_elems
            }
            (Some(last_visitor), None) => {
                vec![T::from(last_visitor)]
            }
            (None, None) => vec![],
            _ => unreachable!(),
        }
    }

    fn pop_front(&mut self) -> Option<Tree<'input, Lang>> {
        match &mut self.cst {
            Some(Tree::Node { children, .. }) => children.pop_front(),
            Some(Tree::Leaf { .. }) => self.cst.take(),
            None => None,
        }
    }

    fn pop_spawn(&mut self) -> Option<CSTreeVisitor<'input, Lang>> {
        match self.pop_front() {
            Some(tree) => {
                // println!("Popping tree: {:?}\n\n", tree);
                Some(CSTreeVisitor { cst: Some(tree) })
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use copager::cfl::{CFLRules, CFLTokens, CFL};
    use copager::template::LALR1;
    use copager::prelude::*;
    use copager::Processor;

    use super::CSTreeVisitor;

    type TestLang = LALR1<TestLangDef>;

    #[derive(Debug, Default, Clone, Copy, CFL)]
    struct TestLangDef (
        #[tokens] TestToken,
        #[rules]  TestRule,
    );

    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, CFLTokens)]
    enum TestToken {
        #[default]
        #[token(r"a")]
        A,
        #[token(r"b")]
        B,
        #[token(r"c")]
        C,
        #[token(r"d")]
        D,
    }

    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, CFLRules)]
    enum TestRule {
        #[default]
        #[rule("<rule_a> ::= A <rule_b>")]
        RuleA,

        #[rule("<rule_b> ::= B <rule_c_list> <rule_d_list>")]
        RuleB,

        #[rule("<rule_c_list> ::= <rule_c_list> <rule_c>")]
        #[rule("<rule_c_list> ::= <rule_c>")]
        RuleCs,

        #[rule("<rule_c> ::= C")]
        RuleC,

        #[rule("<rule_d_list> ::= <rule_d_list> <rule_d>")]
        #[rule("<rule_d_list> ::= ")]
        RuleDs,

        #[rule("<rule_d> ::= D")]
        RuleD,
    }

    #[test]
    fn test_visitor_peek() -> anyhow::Result<()> {
        let mut visitor = Processor::<TestLang>::new()
            .build_lexer()?
            .build_parser()?
            .process::<CSTreeVisitor<_>>("abc")?;

        assert_eq!(visitor.peek(), (Some(TestToken::A), None));
        assert_eq!(visitor.expect_leaf(), (TestToken::A, "a"));
        assert_eq!(visitor.peek(), (None, Some(TestRule::RuleB)));

        Ok(())
    }

    #[test]
    fn test_visitor_expect() -> anyhow::Result<()> {
        #[derive(Debug, PartialEq, Eq)]
        struct AstA;

        impl From<CSTreeVisitor<'_, TestLangDef>> for AstA {
            fn from(mut visitor: CSTreeVisitor<'_, TestLangDef>) -> Self {
                assert_eq!(visitor.expect_leaf(), (TestToken::A, "a"));
                assert_eq!(visitor.expect_node::<AstB>(), AstB);
                AstA
            }
        }

        #[derive(Debug, PartialEq, Eq)]
        struct AstB;

        impl From<CSTreeVisitor<'_, TestLangDef>> for AstB {
            fn from(mut visitor: CSTreeVisitor<'_, TestLangDef>) -> Self {
                assert_eq!(visitor.expect_leaf(), (TestToken::B, "b"));
                assert_eq!(visitor.expect_nodes::<AstC>(), vec![AstC, AstC, AstC]);
                assert_eq!(visitor.expect_nodes::<AstD>(), vec![]);
                AstB
            }
        }

        #[derive(Debug, PartialEq, Eq)]
        struct AstC;

        impl From<CSTreeVisitor<'_, TestLangDef>> for AstC {
            fn from(mut visitor: CSTreeVisitor<'_, TestLangDef>) -> Self {
                assert_eq!(visitor.expect_leaf(), (TestToken::C, "c"));
                AstC
            }
        }

        #[derive(Debug, PartialEq, Eq)]
        struct AstD;

        impl From<CSTreeVisitor<'_, TestLangDef>> for AstD {
            fn from(mut visitor: CSTreeVisitor<'_, TestLangDef>) -> Self {
                assert_eq!(visitor.expect_leaf(), (TestToken::D, "d"));
                AstD
            }
        }

        let visitor = Processor::<TestLang>::new()
            .build_lexer()?
            .build_parser()?
            .process::<CSTreeVisitor<_>>("abccc")?;

        assert_eq!(AstA::from(visitor), AstA);

        Ok(())
    }
}
