pub mod decl;
pub mod r#fn;
pub mod infer;
pub mod op;
pub mod parse;

use std::sync::{Arc, Mutex};

use crate::data::TypeTree;

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
