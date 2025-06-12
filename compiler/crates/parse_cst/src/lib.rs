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
    pub fn expect_leaf(&mut self) -> &'input str {
        match self.pop_front() {
            Some(Tree::Leaf { text, .. }) => text,
            Some(..) => panic!("Expected a leaf but found a node"),
            None => panic!("No more elements in the CSTreeVisitor"),
        }
    }

    pub fn expect_node<T>(&mut self) -> T
    where
        T: From<(String, CSTreeVisitor<'input, Lang>)>,
    {
        match self.pop_spawn() {
            Some(node_visitor) => T::from(("".into(), node_visitor)),
            None => panic!("No more elements in the CSTreeVisitor"),
        }
    }

    pub fn expect_nodes<T>(&mut self) -> Vec<T>
    where
        T: From<(String, CSTreeVisitor<'input, Lang>)>,
        Lang: std::fmt::Debug,
    {
        match self.pop_spawn() {
            Some(mut node_visitor) => node_visitor.expect_nodes_lrec::<T>(),
            None => vec![],
        }
    }

    fn expect_nodes_lrec<T>(&mut self) -> Vec<T>
    where
        T: From<(String, CSTreeVisitor<'input, Lang>)>,
    {
        match (self.pop_spawn(), self.pop_spawn()) {
            (Some(mut lrec_visitor), Some(last_visitor)) => {
                let mut lrec_elems = lrec_visitor.expect_nodes_lrec::<T>();
                let last_elem = T::from(("".into(), last_visitor));
                lrec_elems.push(last_elem);
                lrec_elems
            }
            (Some(last_visitor), None) => {
                vec![T::from(("".into(), last_visitor))]
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
            Some(tree) => Some(CSTreeVisitor { cst: Some(tree) }),
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
        #[tokens] TestTokens,
        #[rules]  TestRules,
    );

    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, CFLTokens)]
    enum TestTokens {
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
    enum TestRules {
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
    fn test_visitor_expect() -> anyhow::Result<()> {
        #[derive(Debug, PartialEq, Eq)]
        struct AstA;

        impl From<(String, CSTreeVisitor<'_, TestLangDef>)> for AstA {
            fn from((_, mut visitor): (String, CSTreeVisitor<'_, TestLangDef>)) -> Self {
                assert_eq!(visitor.expect_leaf(), "a");
                assert_eq!(visitor.expect_node::<AstB>(), AstB);
                AstA
            }
        }

        #[derive(Debug, PartialEq, Eq)]
        struct AstB;

        impl From<(String, CSTreeVisitor<'_, TestLangDef>)> for AstB {
            fn from((_, mut visitor): (String, CSTreeVisitor<'_, TestLangDef>)) -> Self {
                assert_eq!(visitor.expect_leaf(), "b");
                assert_eq!(visitor.expect_nodes::<AstC>(), vec![AstC, AstC, AstC]);
                assert_eq!(visitor.expect_nodes::<AstD>(), vec![]);
                AstB
            }
        }

        #[derive(Debug, PartialEq, Eq)]
        struct AstC;

        impl From<(String, CSTreeVisitor<'_, TestLangDef>)> for AstC {
            fn from((_, mut visitor): (String, CSTreeVisitor<'_, TestLangDef>)) -> Self {
                assert_eq!(visitor.expect_leaf(), "c");
                AstC
            }
        }

        #[derive(Debug, PartialEq, Eq)]
        struct AstD;

        impl From<(String, CSTreeVisitor<'_, TestLangDef>)> for AstD {
            fn from((_, mut visitor): (String, CSTreeVisitor<'_, TestLangDef>)) -> Self {
                assert_eq!(visitor.expect_leaf(), "d");
                AstD
            }
        }

        let visitor = Processor::<TestLang>::new()
            .build_lexer()?
            .build_parser()?
            .process::<CSTreeVisitor<_>>("abccc")?;

        assert_eq!(AstA::from(("".into(), visitor)), AstA);

        Ok(())
    }
}
