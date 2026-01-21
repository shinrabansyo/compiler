mod name;

use std::sync::{Arc, Mutex};

use sb_compiler_semcheck_impl_var::{VarContext, VarGraph};
use sb_compiler_semcheck_impl_typedecl::{TypeDeclChecker, TypeDeclContext};

use name::Name;

#[derive(Clone)]
pub struct SemCheckContext<'src> {
    pub name: Name,
    pub var: VarContext<'src>,
    pub type_decl: TypeDeclContext,
}

#[allow(dead_code)]
pub struct SemCheckServer<'src> {
    var_graph: Arc<Mutex<VarGraph<'src>>>,
    type_decl: Arc<Mutex<TypeDeclChecker>>,
}

impl<'src> SemCheckServer<'src> {
    pub fn new() -> (SemCheckContext<'src>, SemCheckServer<'src>) {
        // 各種意味チェッカの初期化
        let var_graph = VarGraph::new();
        let (type_decl_checker, type_decl_ctx) = TypeDeclChecker::new();

        // コンテキスト，サーバの準備
        let ctx = SemCheckContext {
            name: Name::new::<64>(),
            var: VarContext::from(Arc::clone(&var_graph)),
            type_decl: type_decl_ctx,
        };
        let server = SemCheckServer {
            var_graph,
            type_decl: type_decl_checker,
        };

        (ctx, server)
    }
}
