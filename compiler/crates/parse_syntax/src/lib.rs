use serde::{Serialize, Deserialize};

use copager::cfl::{CFL, CFLTokens, CFLRules};
use copager::template::LALR1;
use copager::prelude::*;

pub type SBLang = LALR1<SBLangDef>;

#[derive(
    Debug, Default, Clone, Copy,
    CFL, Serialize, Deserialize,
)]
pub struct SBLangDef (
    #[tokens] SBTokens,
    #[rules]  SBRules,
);

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash,
    CFLTokens, Serialize, Deserialize,
)]
pub enum SBTokens {
    #[default]

    // 記号
    #[token(r"\+")]
    Plus,
    #[token(r"\-")]
    Minus,
    #[token(r"\(", ir_omit)]
    ParenL,
    #[token(r"\)", ir_omit)]
    ParenR,

    // リテラル
    #[token(r"[0-9]+")]
    Num,

    // 制御用
    #[token(r"[ |\t|\n]+", trivia)]
    Trivia,
    #[token(r".+")]
    Unknown,
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash,
    CFLRules, Serialize, Deserialize,
)]
pub enum SBRules {
    #[default]
    #[rule("<expr> ::= <expr> Plus <value>")]
    #[rule("<expr> ::= <expr> Minus <value>")]
    #[rule("<expr> ::= <value>")]
    Expr,

    #[rule("<value> ::= ParenL <expr> ParenR")]
    #[rule("<value> ::= Num")]
    Value,
}
