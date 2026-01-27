pub mod decl;

use std::sync::{Arc, Mutex};

use sb_compiler_utils::collections::{LayeredGraph, LayeredGraphCursor};

use crate::var::Var;

#[derive(Debug)]
pub struct VarContext<'src> {
    graph: Arc<Mutex<LayeredGraph<Var<'src>>>>,
    cur: LayeredGraphCursor,
}

impl<'src> VarContext<'src> {
    pub fn new() -> Self {
        let (graph, cur) = LayeredGraph::new_with_directed();
        VarContext {
            graph: Arc::new(Mutex::new(graph)),
            cur,
        }
    }
}

impl Clone for VarContext<'_> {
    fn clone(&self) -> Self {
        // Ctx が複製されたとき，新しい変数空間を作成する
        let new_cur = self.graph
            .lock()
            .unwrap()
            .add_directed_layer(self.cur);

        VarContext {
            graph: Arc::clone(&self.graph),
            cur: new_cur,
        }
    }
}
