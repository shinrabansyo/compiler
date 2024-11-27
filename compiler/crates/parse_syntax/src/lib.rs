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
    #[token(r"=", ir_omit)]
    Assign,
    #[token(":", ir_omit)]
    Colon,
    #[token(r";", ir_omit)]
    Semicolon,

    // 予約語
    #[token("const", ir_omit)]
    Const,
    #[token("i32")]
    Type,

    // リテラル
    #[token(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident,
    #[token(r"[0-9]+")]
    Num,

    // 制御用
    #[token(r"[ |\t|\n]+", trivia)]
    Trivia,
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash,
    CFLRules, Serialize, Deserialize,
)]
pub enum SBRules {
    #[default]
    #[rule("<program> ::= <program> <top>")]
    #[rule("<program> ::= <top>")]
    Program,

    #[rule("<top> ::= <const_decl>")]
    Top,

    #[rule("<const_decl> ::= Const Ident Colon Type Assign <expr> Semicolon")]
    ConstDecl,

    #[rule("<expr> ::= <expr> Plus <value>")]
    #[rule("<expr> ::= <expr> Minus <value>")]
    #[rule("<expr> ::= <value>")]
    Expr,

    #[rule("<value> ::= ParenL <expr> ParenR")]
    #[rule("<value> ::= Num")]
    #[rule("<value> ::= Ident")]
    Value,
}
