mod var;

pub mod translate;
pub mod syntax;

pub mod prelude {
    pub use crate::syntax::{LirSyntax, LirSyntaxNode};
    pub use crate::syntax::*;
}
