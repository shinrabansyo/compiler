use thiserror::Error;

use crate::r#type::Type;

#[derive(Error, Debug)]
pub enum TypeError {
    #[error("Callee requires {0} arguments, but got {1}")]
    CalleeArgsMismatch(usize, usize),
    #[error("Type cast failed: {0:?} -> {1:?}")]
    TypeCastFailed(Type, Type),
    #[error("Type mismatch: {0:?} != {1:?}")]
    TypeMismatch(Type, Type),
    #[error("Type inference failed: {0:?}, {1:?}")]
    TypeInfer2Failed(Type, Type),
}

impl TypeError {
    pub fn new_callee_args_mismatch(expected: usize, got: usize) -> miette::Report {
        miette::miette!("Callee requires {} arguments, but got {}", expected, got)
        // TypeError::CalleeArgsMismatch(expected, got)
    }

    pub fn new_cast_failed(a: Type, b: Type) -> miette::Report {
        miette::miette!("Type cast failed: {:?} -> {:?}", a, b)
        // TypeError::TypeCastFailed(a, b)
    }

    pub fn new_mismatch(a: Type, b: Type) -> miette::Report {
        miette::miette!("Type mismatch: {:?} != {:?}", a, b)
        // TypeError::TypeMismatch(a, b)
    }

    pub fn new_infer2_failed(a: Type, b: Type) -> miette::Report {
        miette::miette!("Type inference failed: {:?}, {:?}", a, b)
        // TypeError::TypeInfer2Failed(a, b)
    }
}
