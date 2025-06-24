use std::sync::{Arc, LazyLock};

use super::Typed;

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

impl Typed for Type {
    fn ty(&self) -> Arc<Type> {
        match self {
            // プリミティブ型
            Type::Void => {
                static VOID: LazyLock<Arc<Type>> = LazyLock::new(|| {
                    Arc::new(Type::Void)
                });
                Arc::clone(&VOID)
            }
            Type::Bool => {
                static BOOL: LazyLock<Arc<Type>> = LazyLock::new(|| {
                    Arc::new(Type::Bool)
                });
                Arc::clone(&BOOL)
            }
            Type::I8 => {
                static I8: LazyLock<Arc<Type>> = LazyLock::new(|| {
                    Arc::new(Type::I8)
                });
                Arc::clone(&I8)
            }
            Type::I16 => {
                static I16: LazyLock<Arc<Type>> = LazyLock::new(|| {
                    Arc::new(Type::I16)
                });
                Arc::clone(&I16)
            }
            Type::I32 => {
                static I32: LazyLock<Arc<Type>> = LazyLock::new(|| {
                    Arc::new(Type::I32)
                });
                Arc::clone(&I32)
            }
            Type::NumConst => {
                static NUM_CONST: LazyLock<Arc<Type>> = LazyLock::new(|| {
                    Arc::new(Type::NumConst)
                });
                Arc::clone(&NUM_CONST)
            },

            // 関数
            Type::Function { args, ret_ty } => {
                Arc::new(Type::Function {
                    args: args.iter().map(Arc::clone).collect(),
                    ret_ty: Arc::clone(ret_ty),
                })
            }
        }
    }
}

impl<T: Typed> Typed for Box<T> {
    fn ty(&self) -> Arc<Type> {
        self.as_ref().ty()
    }
}

impl<T: Typed> Typed for Arc<T> {
    fn ty(&self) -> Arc<Type> {
        self.as_ref().ty()
    }
}
