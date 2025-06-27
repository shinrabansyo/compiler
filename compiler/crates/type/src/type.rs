use std::sync::{Arc, LazyLock};

use sb_compiler_parse_ast as ast;

use super::Typed;

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

    // 関数
    Function {
        args: Vec<Arc<Type>>,
        ret_ty: Arc<Type>,
    },
}

impl<'src> From<ast::Type<'src>> for Type {
    fn from(ast: ast::Type<'src>) -> Self {
        match ast {
            // プリミティブ
            ast::Type::Bool(_) => Type::Bool,
            ast::Type::Char(_) => Type::Char,
            ast::Type::I8(_) => Type::I8,
            ast::Type::I16(_) => Type::I16,
            ast::Type::I32(_) => Type::I32,

            _ => panic!(""),
        }
    }
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
