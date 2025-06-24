use std::sync::Arc;

pub use Type::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    // プリミティブ
    Void,
    Bool,
    I8,
    I16,
    I32,
    NumConst,

    // 関数
    Function {
        args: Vec<Arc<Type>>,
        ret_ty: Arc<Type>,
    },
}
