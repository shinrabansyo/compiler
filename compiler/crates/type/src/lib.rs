mod error;
pub mod op;
pub mod r#type;

pub trait Typed {
    fn ty(&self) -> &r#type::Type;
}
