mod primitive;

use std::sync::Arc;

#[allow(ambiguous_glob_reexports)]
pub use primitive::*;
pub use Type::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Function {
        args: Vec<Arc<Type>>,
        ret_ty: Arc<Type>,
    },
    Primitive(primitive::Primitive),
}
