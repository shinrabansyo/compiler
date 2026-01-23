use std::sync::{Arc, LazyLock};

pub trait Typed {
    fn ty(&self) -> Arc<Type>;
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

pub use Type::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    // プリミティブ
    Void,
    Bool,
    Char,
    I8,
    I16,
    I32,
    NumConst,

    // アドレス
    Addr(Arc<Type>),
    DataAddr(Arc<Type>),
    InstAddr(Arc<Type>),

    // 関数
    Function {
        args: Vec<Arc<Type>>,
        ret_ty: Arc<Type>,
    },
}

impl Typed for Type {
    fn ty(&self) -> Arc<Type> {
        match self {
            // プリミティブ
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
            Type::Char => {
                static CHAR: LazyLock<Arc<Type>> = LazyLock::new(|| {
                    Arc::new(Type::Char)
                });
                Arc::clone(&CHAR)
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

            // アドレス
            Type::Addr(inner_ty) => {
                Arc::new(Type::Addr(Arc::clone(inner_ty)))
            }
            Type::DataAddr(inner_ty) => {
                Arc::new(Type::DataAddr(Arc::clone(inner_ty)))
            }
            Type::InstAddr(inner_ty) => {
                Arc::new(Type::InstAddr(Arc::clone(inner_ty)))
            }

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
