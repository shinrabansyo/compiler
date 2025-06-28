// 判断
mod cast;       pub use cast::ty_cast;
mod equals;     pub use equals::{ty_equals, ty_equals_arith2};

// 演算
mod det;        pub use det::ty_det_arith2;
mod infer;      pub use infer::ty_infer;

// 関数関連
mod call;       pub use call::ty_can_call;
mod r#return;   pub use r#return::ty_can_return;
