use thiserror::Error;

use crate::r#type::Type;

#[derive(Error, Debug)]
pub enum TypeError {
    #[error("Type cast failed: {0:?} -> {1:?}")]
    TypeCastFailed(Type, Type),
    #[error("Type mismatch: {0:?} != {1:?}")]
    TypeMismatch(Type, Type),
    #[error("Type inference failed: {0:?}, {1:?}")]
    TypeInfer2Failed(Type, Type),
}

impl TypeError {
    pub fn new_cast_failed(a: Type, b: Type) -> Self {
        TypeError::TypeCastFailed(a, b)
    }

    pub fn new_mismatch(a: Type, b: Type) -> Self {
        TypeError::TypeMismatch(a, b)
    }

    pub fn new_infer2_failed(a: Type, b: Type) -> Self {
        TypeError::TypeInfer2Failed(a, b)
    }
}
