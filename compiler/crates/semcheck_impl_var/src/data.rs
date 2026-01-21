use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use petgraph::graph::{Graph, NodeIndex};

use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_type::r#type::*;
use sb_compiler_type::Typed;


#[derive(Debug, Clone, Hash)]
pub struct Var<'src> {
    pub id: NodeIndex,
    pub span: Span<'src>,
    pub ty: Arc<Type>,
}

impl PartialEq for Var<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Var<'_> {}

impl<'src> Spanned<'src> for Var<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}

impl Typed for Var<'_> {
    fn ty(&self) -> Arc<Type> {
        self.ty.ty()
    }
}

#[derive(Debug)]
pub struct VarGraph<'src> {
    pub(crate) graph: Graph<Var<'src>, ()>,
    pub(crate) root_node: NodeIndex,
    pub(crate) nodes: HashMap<Var<'src>, NodeIndex>,
}

impl<'src> VarGraph<'src> {
    pub fn new() -> Arc<Mutex<VarGraph<'src>>> {
        // ルート用の疑似変数
        let root_span = Span {
            src: ".",
            body: (0, 1),
            full: (0, 1),
        };
        let root_var = Var {
            id: NodeIndex::new(0),
            span: root_span,
            ty: Arc::new(I32),
        };

        // 変数参照グラフ
        let mut graph = Graph::new();
        let root_node = graph.add_node(root_var.clone());
        graph.node_weight_mut(root_node).unwrap().id = root_node;

        Arc::new(Mutex::new(VarGraph {
            graph,
            root_node,
            nodes: HashMap::from([(root_var, root_node)]),
        }))
    }
}
