pub mod primitive;

pub use Type::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Primitive(primitive::Primitive),
}
