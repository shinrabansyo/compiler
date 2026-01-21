mod data;
mod func;
mod error;

pub use data::{Var, VarGraph};
pub use func::{Context as VarContext, var_find, var_register};
pub use error::VarError;
