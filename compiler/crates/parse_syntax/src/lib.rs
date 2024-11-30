use serde::{Serialize, Deserialize};

use copager::cfl::{CFLRules, CFLTokens, CFL};
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

    // 演算子
    #[token(r"==")]
    Eq,
    #[token(r"!=")]
    Neq,
    #[token(r"=")]
    Assign,
    #[token(r"\+=")]
    PlusAssign,
    #[token(r"\-=")]
    MinusAssign,
    #[token(r"<<=")]
    ShiftLAssign,
    #[token(r">>>=")]
    ShiftRaAssign,
    #[token(r">>=")]
    ShiftRAssign,
    #[token(r"lor")]
    LogicOr,
    #[token(r"land")]
    LogicAnd,
    #[token(r"bor")]
    BitOr,
    #[token(r"bxor")]
    BitXor,
    #[token(r"band")]
    BitAnd,
    #[token(r"<<")]
    ShiftL,
    #[token(r">>>")]
    ShiftRa,
    #[token(r">>")]
    ShiftR,
    #[token(r"<=")]
    Lte,
    #[token(r"<")]
    Lt,
    #[token(r">=")]
    Gte,
    #[token(r">")]
    Gt,
    #[token(r"\+")]
    Plus,
    #[token(r"\-")]
    Minus,

    // 記号
    #[token(r"->", ir_omit)]
    Allow,
    #[token(r",", ir_omit)]
    Comma,
    #[token(r"\(", ir_omit)]
    ParenL,
    #[token(r"\)", ir_omit)]
    ParenR,
    #[token(r"\{", ir_omit)]
    BraceL,
    #[token(r"\}", ir_omit)]
    BraceR,
    #[token(":", ir_omit)]
    Colon,
    #[token(r";", ir_omit)]
    Semicolon,

    // 予約語
    #[token("fn" ir_omit)]
    Fn,
    #[token("var", ir_omit)]
    Var,
    #[token("return", ir_omit)]
    Return,
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

    #[rule("<top> ::= <var_decl>")]
    #[rule("<top> ::= <func_def>")]
    Top,

    // 定義
    #[rule("<func_def> ::= Fn Ident ParenL <arg_def_list> ParenR <block>")]
    #[rule("<func_def> ::= Fn Ident ParenL <arg_def_list> ParenR Allow Type <block>")]
    #[rule("<arg_def_list> ::= <arg_def_list> Comma <arg_def>")]
    #[rule("<arg_def_list> ::= <arg_def>")]
    #[rule("<arg_def_list> ::= ")]
    FuncDef,

    #[rule("<arg_def> ::= Ident Colon Type")]
    ArgumentDef,

    // 文
    #[rule("<stmt> ::= <var_decl>")]
    #[rule("<stmt> ::= <block>")]
    #[rule("<stmt> ::= <expr> Semicolon")]
    #[rule("<stmt> ::= <return> Semicolon")]
    Stmt,

    #[rule("<block> ::= BraceL <stmt_list> BraceR")]
    #[rule("<stmt_list> ::= <stmt_list> <stmt>")]
    #[rule("<stmt_list> ::= <stmt>")]
    Block,

    #[rule("<var_decl> ::= Var Ident Colon Type Assign <expr> Semicolon")]
    VarDecl,

    #[rule("<return> ::= Return <expr>")]
    Return,

    // 式
    #[rule("<expr> ::= <assign>")]
    Expr,

    #[rule("<assign> ::= Ident Assign <assign>")]
    #[rule("<assign> ::= Ident PlusAssign <assign>")]
    #[rule("<assign> ::= Ident MinusAssign <assign>")]
    #[rule("<assign> ::= Ident ShiftLAssign <assign>")]
    #[rule("<assign> ::= Ident ShiftRaAssign <assign>")]
    #[rule("<assign> ::= Ident ShiftRAssign <assign>")]
    #[rule("<assign> ::= <logic_or>")]
    Assign,

    #[rule("<logic_or> ::= <logic_or> LogicOr <logic_and>")]
    #[rule("<logic_or> ::= <logic_and>")]
    LogicOr,

    #[rule("<logic_and> ::= <logic_and> LogicAnd <bit_or>")]
    #[rule("<logic_and> ::= <bit_or>")]
    LogicAnd,

    #[rule("<bit_or> ::= <bit_or> BitOr <bit_xor>")]
    #[rule("<bit_or> ::= <bit_xor>")]
    BitOr,

    #[rule("<bit_xor> ::= <bit_xor> BitXor <bit_and>")]
    #[rule("<bit_xor> ::= <bit_and>")]
    BitXor,

    #[rule("<bit_and> ::= <bit_and> BitAnd <cond>")]
    #[rule("<bit_and> ::= <cond>")]
    BitAnd,

    #[rule("<cond> ::= <cond> Eq <bit_shift>")]
    #[rule("<cond> ::= <cond> Neq <bit_shift>")]
    #[rule("<cond> ::= <cond> Lt <bit_shift>")]
    #[rule("<cond> ::= <cond> Lte <bit_shift>")]
    #[rule("<cond> ::= <cond> Gt <bit_shift>")]
    #[rule("<cond> ::= <cond> Gte <bit_shift>")]
    #[rule("<cond> ::= <bit_shift>")]
    Cond,

    #[rule("<bit_shift> ::= <bit_shift> ShiftL <add>")]
    #[rule("<bit_shift> ::= <bit_shift> ShiftR <add>")]
    #[rule("<bit_shift> ::= <bit_shift> ShiftRa <add>")]
    #[rule("<bit_shift> ::= <add>")]
    BitShift,

    #[rule("<add> ::= <add> Plus <value>")]
    #[rule("<add> ::= <add> Minus <value>")]
    #[rule("<add> ::= <value>")]
    Add,

    #[rule("<value> ::= Num")]
    #[rule("<value> ::= Ident")]
    #[rule("<value> ::= ParenL <expr> ParenR")]
    #[rule("<value> ::= <call>")]
    Value,

    #[rule("<call> ::= Ident ParenL <arg_list> ParenR")]
    #[rule("<arg_list> ::= <arg_list> Comma <value>")]
    #[rule("<arg_list> ::= <value>")]
    #[rule("<arg_list> ::= ")]
    Call,
}
