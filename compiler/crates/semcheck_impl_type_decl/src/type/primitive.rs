pub use Primitive::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Primitive {
    Void,
    I8,
    I32,
}
