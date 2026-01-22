mod ty_register;
mod ty_find;

use std::sync::{Arc, Mutex};

use crate::data::TypeTree;

pub use ty_register::{ty_register, ty_register_in_mod};
pub use ty_find::{ty_find, ty_find_from_mod};

#[derive(Clone)]
pub struct TypeContext {
    tree: Arc<Mutex<TypeTree>>,
    pub(crate) current_pos: usize,
}

impl From<Arc<Mutex<TypeTree>>> for TypeContext {
    fn from(tree: Arc<Mutex<TypeTree>>) -> Self {
        let root_node = tree.lock().unwrap().root_node;
        TypeContext {
            tree,
            current_pos: root_node,
        }
    }
}
