mod inst;       pub use inst::*;
mod assign;     pub use assign::Assign;
mod terminal;   pub use terminal::Terminal;

use crate::translate::Translatable;

pub trait LirSyntax
where
    Self: Sized + Translatable,
{}

impl<T: Translatable> LirSyntax for T {}
