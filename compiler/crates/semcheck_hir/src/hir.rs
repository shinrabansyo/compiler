// トップ要素
mod program;            pub use program::Program;
mod top;                pub use top::Top;

// 定義
mod struct_def;         pub use struct_def::StructDef;
mod field_def;          pub use field_def::FieldDef;
mod func_def;           pub use func_def::FuncDef;
mod argument_def;       pub use argument_def::ArgumentDef;

// 文
mod block;              pub use block::Block;
mod stmt;               pub use stmt::Stmt;
mod var_decl;           pub use var_decl::VarDecl;
mod r#return;           pub use r#return::Return;
mod r#if;               pub use r#if::If;
mod r#while;            pub use r#while::While;
mod r#for;              pub use r#for::For;
mod inasm;              pub use inasm::InlineAsm;
mod inasm_inst;         pub use inasm_inst::InlineAsmInst;
mod inasm_operand;      pub use inasm_operand::InlineAsmOperand;

// 式
mod expr;               pub use expr::Expr;
mod assign;             pub use assign::Assign;
mod logic_or;           pub use logic_or::LogicOr;
mod logic_and;          pub use logic_and::LogicAnd;
mod bit_or;             pub use bit_or::BitOr;
mod bit_xor;            pub use bit_xor::BitXor;
mod bit_and;            pub use bit_and::BitAnd;
mod cond;               pub use cond::Cond;
mod bit_shift;          pub use bit_shift::BitShift;
mod add;                pub use add::Add;
mod mul;                pub use mul::Mul;
mod cast;               pub use cast::Cast;
mod unary;              pub use unary::Unary;
mod value_r;            pub use value_r::ValueR;
mod struct_init;        pub use struct_init::StructInit;
mod struct_field_init;  pub use struct_field_init::StructFieldInit;
mod struct_access;      pub use struct_access::StructAccess;

// AST -> HIR 用トレイト
use std::future::Future;
use std::pin::Pin;
use sb_compiler_semcheck_impl::SemCheckContext;

pub type Dep<'a, 'src> = &'a mut SemCheckContext<'src>;
pub type InDep<'src> = SemCheckContext<'src>;

pub trait SemCheck<Ctx, T> {
    fn check(ctx: Ctx, ast: T) -> Pin<Box<impl Future<Output = miette::Result<Self>>>>
    where
        Self: Sized,
    {
        Box::pin(Self::check0(ctx, ast))
    }

    fn check0(ctx: Ctx, ast: T) -> impl Future<Output = miette::Result<Self>>
    where
        Self: Sized;
}
