pub mod inst;
pub mod var;
pub mod syntax;

pub mod prelude {
    pub use crate::syntax::{LirSyntax, LirSyntaxNode};
    pub use crate::syntax::*;
}
