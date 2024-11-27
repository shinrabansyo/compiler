mod ast;        pub use ast::lirgen_ast;
mod top;        pub use top::lirgen_top;
mod const_decl; pub use const_decl::lirgen_const_decl;
mod expr;       pub use expr::lirgen_expr;
mod value;      pub use value::lirgen_value;

const ZERO_REG: u8  = 0;  // r0
const TMP_REG: u8   = 4;  // r4
const TMP_REG_L: u8 = 4;  // r4
const TMP_REG_R: u8 = 5;  // r5
