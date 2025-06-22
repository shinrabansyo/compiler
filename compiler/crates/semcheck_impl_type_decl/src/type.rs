pub mod primitive;

pub use Type::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type {
    Primitive(primitive::Primitive),
}
