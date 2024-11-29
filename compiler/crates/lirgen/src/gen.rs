mod program;    pub use program::lirgen_program;
mod top;        pub use top::lirgen_top;
mod func_def;   pub use func_def::lirgen_func_def;
mod stmt;       pub use stmt::lirgen_stmt;
mod block;      pub use block::lirgen_block;
mod const_decl; pub use const_decl::lirgen_const_decl;
mod expr;       pub use expr::lirgen_expr;
mod value;      pub use value::lirgen_value;

const ZERO_REG   : u8 = 0;  // r0
const TMP_REG    : u8 = 4;  // r4
const TMP_REG_L  : u8 = 4;  // r4
const TMP_REG_R  : u8 = 5;  // r5
const VARBASE_REG: u8 = 6;  // r6
const RET_REG    : u8 = 10; // r10
