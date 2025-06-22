pub use Primitive::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Primitive {
    // ユーザに見える型
    Void,
    I8,
    I16,
    I32,

    // 解析中に使用する型
    NumConst,
}
