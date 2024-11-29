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
    #[token(r"\{", ir_omit)]
    BraceL,
    #[token(r"\}", ir_omit)]
    BraceR,
    #[token(r"=", ir_omit)]
    Assign,
    #[token(":", ir_omit)]
    Colon,
    #[token(r";", ir_omit)]
    Semicolon,

    // 予約語
    #[token("fn" ir_omit)]
    Fn,
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
    #[rule("<top> ::= <func_def>")]
    Top,

    // 定義
    #[rule("<func_def> ::= Fn Ident ParenL ParenR <block>")]
    FuncDef,

    // 文
    #[rule("<stmt> ::= <const_decl>")]
    #[rule("<stmt> ::= <block>")]
    #[rule("<stmt> ::= <expr> Semicolon")]
    Stmt,

    #[rule("<block> ::= BraceL <stmt_list> BraceR")]
    #[rule("<stmt_list> ::= <stmt_list> <stmt>")]
    #[rule("<stmt_list> ::= <stmt>")]
    Block,

    #[rule("<const_decl> ::= Const Ident Colon Type Assign <expr> Semicolon")]
    ConstDecl,

    // 式
    #[rule("<expr> ::= <expr> Plus <value>")]
    #[rule("<expr> ::= <expr> Minus <value>")]
    #[rule("<expr> ::= <value>")]
    Expr,

    #[rule("<value> ::= Num")]
    #[rule("<value> ::= Ident")]
    #[rule("<value> ::= ParenL <expr> ParenR")]
    #[rule("<value> ::= <call>")]
    Value,

    #[rule("<call> ::= Ident ParenL ParenR")]
    Call,
}
