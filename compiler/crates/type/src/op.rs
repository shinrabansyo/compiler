mod call;
mod cast;
mod equals;
mod infer;
mod r#return;

pub use call::ty_can_call;
pub use cast::ty_cast;
pub use equals::{ty_equals, ty_equals2};
pub use infer::{ty_infer, ty_infer2};
pub use r#return::ty_can_return;

