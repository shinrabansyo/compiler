pub mod decl;

use std::sync::{Arc, Mutex};

use petgraph::graph::NodeIndex;

use crate::data::VarGraph;

#[derive(Debug, Clone)]
pub struct VarContext<'src> {
    graph: Arc<Mutex<VarGraph<'src>>>,
    node: NodeIndex,
}

impl<'src> From<Arc<Mutex<VarGraph<'src>>>> for VarContext<'src> {
    fn from(graph: Arc<Mutex<VarGraph<'src>>>) -> Self {
        let node = graph.lock().unwrap().root_node;
        VarContext { graph, node }
    }
}
