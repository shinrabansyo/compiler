use copager::cfl::{CFL, CFLTokens, CFLRules};
use copager::template::LALR1;
use copager::prelude::*;

pub type SBLang = LALR1<SBLangDef>;

#[derive(Debug, Default, CFL)]
pub struct SBLangDef (
    #[tokens] SBTokens,
    #[rules]  SBRules,
);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, CFLTokens)]
pub enum SBTokens {
    #[default]
    #[token("[0-9]+")]
    Num,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, CFLRules)]
pub enum SBRules {
    #[default]
    #[rule("<expr> ::= Num")]
    Expr,
}
