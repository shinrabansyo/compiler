mod name;

use std::sync::{Arc, Mutex};

use sb_compiler_semcheck_impl_var::{VarContext, VarGraph};
use sb_compiler_semcheck_impl_type::{TypeContext, TypeTree};

use name::Name;

#[derive(Clone)]
pub struct SemCheckContext<'src> {
    pub name: Name,
    pub var: VarContext<'src>,
    pub r#type: TypeContext,
}

#[allow(dead_code)]
pub struct SemCheckDataStore<'src> {
    var_graph: Arc<Mutex<VarGraph<'src>>>,
    type_tree: Arc<Mutex<TypeTree>>,
}

impl<'src> SemCheckDataStore<'src> {
    pub fn new() -> (SemCheckContext<'src>, SemCheckDataStore<'src>) {
        // 各種意味チェッカの初期化
        let var_graph = VarGraph::new();
        let type_tree = TypeTree::new();

        // 意味解析用コンテキスト，データの準備
        let ctx = SemCheckContext {
            name: Name::new::<64>(),
            var: VarContext::from(Arc::clone(&var_graph)),
            r#type: TypeContext::from(Arc::clone(&type_tree)),
        };
        let server = SemCheckDataStore { var_graph, type_tree };

        (ctx, server)
    }
}
