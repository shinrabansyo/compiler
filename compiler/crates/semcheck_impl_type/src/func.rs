mod ty_register;
mod ty_find;

use std::sync::{Arc, Mutex};

use crate::data::TypeTree;

pub use ty_register::ty_register;
pub use ty_find::ty_find;

#[derive(Clone)]
pub struct Context {
    tree: Arc<Mutex<TypeTree>>,
    pub(crate) current_pos: usize,
}

impl From<Arc<Mutex<TypeTree>>> for Context {
    fn from(tree: Arc<Mutex<TypeTree>>) -> Self {
        let root_node = tree.lock().unwrap().root_node;
        Context {
            tree,
            current_pos: root_node,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use sb_compiler_semcheck_async::block_on;
//     use sb_compiler_type::r#type::*;

//     use super::TypeDeclChecker;

//     #[test]
//     fn test_ok() {
//         let (_, ctx) = TypeDeclChecker::new();

//         block_on(async {
//             assert_eq!(*TypeDeclChecker::find(&ctx, "i32").await.unwrap(), I32);
//         });
//     }
// }
