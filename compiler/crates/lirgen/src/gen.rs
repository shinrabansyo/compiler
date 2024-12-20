mod program;    pub use program::lirgen_program;
mod top;        pub use top::lirgen_top;
mod func_def;   pub use func_def::lirgen_func_def;
mod stmt;       pub use stmt::lirgen_stmt;
mod block;      pub use block::lirgen_block;
mod var_decl;   pub use var_decl::lirgen_var_decl;
mod r#if;       pub use r#if::lirgen_if;
mod r#while;    pub use r#while::lirgen_while;
mod r#for;      pub use r#for::lirgen_for;
mod dev_io;     pub use dev_io::lirgen_dev_io;
mod expr;       pub use expr::lirgen_expr;
mod assign;     pub use assign::lirgen_assign;
mod logic_or;   pub use logic_or::lirgen_logic_or;
mod logic_and;  pub use logic_and::lirgen_logic_and;
mod bit_or;     pub use bit_or::lirgen_bit_or;
mod bit_xor;    pub use bit_xor::lirgen_bit_xor;
mod bit_and;    pub use bit_and::lirgen_bit_and;
mod cond;       pub use cond::lirgen_cond;
mod bit_shift;  pub use bit_shift::lirgen_bit_shift;
mod add;        pub use add::lirgen_add;
mod value;      pub use value::lirgen_value;

const ZERO_REG   : u8 = 0;  // r0
const TMP_REG    : u8 = 4;  // r4
const TMP_REG_L  : u8 = 4;  // r4
const TMP_REG_R  : u8 = 5;  // r5
const VARBASE_REG: u8 = 6;  // r6
const RET_REG    : u8 = 10; // r10
