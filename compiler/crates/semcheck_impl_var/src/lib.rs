mod data;
mod func;
mod error;

pub use data::{Var, VarGraph};
pub use func::{VarContext, var_find, var_register};
pub use error::VarDeclError;
