use std::collections::BTreeMap;
use std::fmt::Display;
use std::sync::{Arc, LazyLock};

pub trait Typed {
    fn ty(&self) -> Arc<Type>;
}

impl<T: Typed> Typed for Box<T> {
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

    // データ構造
    Struct {
        name: String,
        fields: BTreeMap<String, Arc<Type>>,
    },

    // 関数
    Function {
        name: String,
        args: Vec<Arc<Type>>,
        ret_ty: Arc<Type>,
    },
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // プリミティブ
            Type::Void => write!(f, "void"),
            Type::Bool => write!(f, "bool"),
            Type::Char => write!(f, "char"),
            Type::I8 => write!(f, "i8"),
            Type::I16 => write!(f, "i16"),
            Type::I32 => write!(f, "i32"),
            Type::NumConst => write!(f, "const(num)"),

            // アドレス
            Type::Addr(inner_ty) => write!(f, "Addr<{}>", inner_ty),
            Type::DataAddr(inner_ty) => write!(f, "DataAddr<{}>", inner_ty),
            Type::InstAddr(inner_ty) => write!(f, "InstAddr<{}>", inner_ty),

            // データ構造
            Type::Struct { name, .. } => {
                write!(f, "struct {}", name)
            }

            // 関数
            Type::Function { name, args, ret_ty } => {
                write!(f, "fn {}(", name)?;
                for arg_ty in args {
                    write!(f, "{}, ", arg_ty)?;
                }
                write!(f, ") -> {}", ret_ty)
            }
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

            // データ構造
            Type::Struct { name, fields } => {
                let fields = fields.iter()
                    .map(|(name, ty)| (name.clone(), Arc::clone(ty)))
                    .collect();
                Arc::new(Type::Struct { name: name.clone(), fields })
            }

            // 関数
            Type::Function { name, args, ret_ty } => {
                Arc::new(Type::Function {
                    name: name.clone(),
                    args: args.iter().map(Arc::clone).collect(),
                    ret_ty: Arc::clone(ret_ty),
                })
            }
        }
    }
}

impl Typed for Arc<Type> {
    fn ty(&self) -> Arc<Type> {
        Arc::clone(self)
    }
}

impl Type {
    pub fn size(&self) -> u32 {
        match self {
            // プリミティブ
            Type::Void => 1,
            Type::Bool => 1,
            Type::Char => 1,
            Type::I8 => 1,
            Type::I16 => 2,
            Type::I32 => 4,
            Type::NumConst => 4,

            // アドレス
            Type::Addr(_) => 4,
            Type::DataAddr(_) => 4,
            Type::InstAddr(_) => 4,

            // データ構造
            Type::Struct { fields, .. } => {
                let mut size = 0;
                for ty in fields.values() {
                    size += ty.size();
                }
                size
            }

            // 関数
            Type::Function { .. } => 4,
        }
    }
}
