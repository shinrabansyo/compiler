mod data;
mod func;
mod error;

pub use data::TypeTree;
pub use func::{TypeContext, ty_register, ty_find};
pub use error::TypeDeclError;
