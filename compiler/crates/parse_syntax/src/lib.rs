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
    #[tokens] SBToken,
    #[rules]  SBRule,
);

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash,
    CFLTokens, Serialize, Deserialize,
)]
pub enum SBToken {
    #[default]

    // 記号
    #[token(r"->", ir_omit)]
    Allow,
    #[token(r",", ir_omit)]
    Comma,
    #[token(r"\[", ir_omit)]
    BracketL,
    #[token(r"\]", ir_omit)]
    BracketR,
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
    #[token(r"fn" ir_omit)]
    Fn,
    #[token(r"var", ir_omit)]
    Var,
    #[token(r"return", ir_omit)]
    Return,
    #[token(r"if", ir_omit)]
    If,
    #[token(r"else", ir_omit)]
    Else,
    #[token(r"while", ir_omit)]
    While,
    #[token(r"for", ir_omit)]
    For,
    #[token(r"asm!", ir_omit)]
    Asm,
    #[token(r"bool")]
    BoolTy,
    #[token(r"char")]
    CharTy,
    #[token(r"i8")]
    I8Ty,
    #[token(r"i16")]
    I16Ty,
    #[token(r"i32")]
    I32Ty,
    #[token(r"Addr")]
    AddrTy,
    #[token(r"DataAddr")]
    DataAddrTy,
    #[token(r"InstAddr")]
    InstAddrTy,

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
    #[token(r"\|\|")]
    LogicOr,
    #[token(r"&&")]
    LogicAnd,
    #[token(r"\|")]
    BitOr,
    #[token(r"\^")]
    BitXor,
    #[token(r"&")]
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
    #[token(r"!")]
    Not,
    #[token(r"\+")]
    Plus,
    #[token(r"\-")]
    Minus,
    #[token(r"\*")]
    Mul,
    #[token(r"/")]
    Div,
    #[token(r"%")]
    Mod,
    #[token(r"as", ir_omit)]
    As,

    // リテラル
    #[token(r"true")]
    True,
    #[token(r"false")]
    False,
    #[token(r"'([a-zA-Z0-9_@]|\\n|\\t)'")]
    Char,
    #[token(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident,
    #[token(r"0b[01]+")]
    #[token(r"0[0-7]+")]
    #[token(r"0x[0-9a-fA-F]+")]
    #[token(r"[0-9]+")]
    Num,

    // 制御用
    #[token(r"^( |\t|\n|(//(.*)\n))*", pre_trivia)]
    #[token(r"^( |\t|)*(//(.*)\n)", post_trivia)]
    Trivia,
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash,
    CFLRules, Serialize, Deserialize,
)]
pub enum SBRule {
    #[default]
    #[rule("<program> ::= <top_list>")]
    Program,

    #[rule("<top_list> ::= <top_list> <top>")]
    #[rule("<top_list> ::= <top>")]
    #[rule("<top> ::= <func_def>")]
    Top,

    // 定義
    #[rule("<func_def> ::= Fn Ident ParenL <arg_def_list> ParenR <block>")]
    #[rule("<func_def> ::= Fn Ident ParenL <arg_def_list> ParenR Allow <type> <block>")]
    FuncDef,

    #[rule("<arg_def_list> ::= <arg_def_list> Comma <arg_def>")]
    #[rule("<arg_def_list> ::= <arg_def>")]
    #[rule("<arg_def_list> ::= ")]
    #[rule("<arg_def> ::= Ident Colon <type>")]
    ArgumentDef,

    // 型
    #[rule("<type> ::= BoolTy")]
    #[rule("<type> ::= CharTy")]
    #[rule("<type> ::= I8Ty")]
    #[rule("<type> ::= I16Ty")]
    #[rule("<type> ::= I32Ty")]
    #[rule("<type> ::= AddrTy Lt <type> Gt")]
    #[rule("<type> ::= DataAddrTy Lt <type> Gt")]
    #[rule("<type> ::= InstAddrTy Lt <type> Gt")]
    Type,

    // 文
    #[rule("<block> ::= BraceL <stmt_list> BraceR")]
    Block,

    #[rule("<stmt_list> ::= <stmt_list> <stmt>")]
    #[rule("<stmt_list> ::= <stmt>")]
    #[rule("<stmt> ::= <var_decl> Semicolon")]
    #[rule("<stmt> ::= <block>")]
    #[rule("<stmt> ::= <expr> Semicolon")]
    #[rule("<stmt> ::= <return> Semicolon")]
    #[rule("<stmt> ::= <if>")]
    #[rule("<stmt> ::= <while>")]
    #[rule("<stmt> ::= <for>")]
    #[rule("<stmt> ::= <inasm>")]
    Stmt,

    #[rule("<var_decl> ::= Var Ident Colon <type> Assign <expr>")]
    #[rule("<var_decl> ::= Var Ident Assign <expr>")]
    VarDecl,

    #[rule("<return> ::= Return <expr>")]
    Return,

    #[rule("<if> ::= If ParenL <expr> ParenR <block>")]
    #[rule("<if> ::= If ParenL <expr> ParenR <block> Else <stmt>")]
    If,

    #[rule("<while> ::= While ParenL <expr> ParenR <block>")]
    While,

    #[rule("<for> ::= For ParenL <var_decl> Semicolon <expr> Semicolon <expr> ParenR <block>")]
    For,

    #[rule("<inasm> ::= Asm BraceL <inasm_inst_list> BraceR")]
    InlineAsm,

    #[rule("<inasm_inst_list> ::= <inasm_inst_list> <inasm_inst>")]
    #[rule("<inasm_inst_list> ::= <inasm_inst>")]
    #[rule("<inasm_inst> ::= Ident Ident Assign Ident Comma Num")]                          // I-形式
    #[rule("<inasm_inst> ::= Ident Ident Comma Ident BracketL Num BracketR")]               // I-形式 (jal)
    #[rule("<inasm_inst> ::= Ident Ident Assign Ident BracketL Num BracketR")]              // I-形式 (lw ...)
    #[rule("<inasm_inst> ::= Ident Ident BracketL Num BracketR Assign Ident")]              // S-形式
    #[rule("<inasm_inst> ::= Ident Ident Assign Ident Comma Ident")]                        // R-形式
    #[rule("<inasm_inst> ::= Ident Ident Comma ParenL Ident Comma Ident ParenR Allow Num")] // B-形式
    InlineAsmInst,

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

    #[rule("<add> ::= <add> Plus <mul>")]
    #[rule("<add> ::= <add> Minus <mul>")]
    #[rule("<add> ::= <mul>")]
    Add,

    #[rule("<mul> ::= <mul> Mul <cast>")]
    #[rule("<mul> ::= <mul> Div <cast>")]
    #[rule("<mul> ::= <mul> Mod <cast>")]
    #[rule("<mul> ::= <cast>")]
    Mul,

    #[rule("<cast> ::= <unary> As <type>")]
    #[rule("<cast> ::= <unary>")]
    Cast,

    #[rule("<unary> ::= Not <value>")]
    #[rule("<unary> ::= Plus <value>")]
    #[rule("<unary> ::= Minus <value>")]
    #[rule("<unary> ::= <value>")]
    Unary,

    #[rule("<value_list> ::= <value_list> Comma <value>")]
    #[rule("<value_list> ::= <value>")]
    #[rule("<value_list> ::= ")]
    #[rule("<value> ::= True")]
    #[rule("<value> ::= False")]
    #[rule("<value> ::= Char")]
    #[rule("<value> ::= Num")]
    #[rule("<value> ::= Ident")]
    #[rule("<value> ::= Ident ParenL <value_list> ParenR")]
    #[rule("<value> ::= ParenL <expr> ParenR")]
    Value,
}
