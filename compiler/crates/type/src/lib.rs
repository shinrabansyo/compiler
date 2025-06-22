mod primitive;

#[allow(ambiguous_glob_reexports)]
pub use primitive::*;
pub use Type::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type {
    Primitive(primitive::Primitive),
}

pub trait Typed {
    fn ty(&self) -> &Type;
}
