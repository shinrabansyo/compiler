mod name;

use std::sync::{Arc, Mutex};

use sb_compiler_semcheck_impl_vardecl::{VarDeclChecker, VarDeclContext};

use name::Name;

#[derive(Debug, Clone)]
pub struct SemCheckContext<'src> {
    pub name: Name,
    pub var_decl: VarDeclContext<'src>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct SemCheckServer<'src> {
    var_decl: Arc<Mutex<VarDeclChecker<'src>>>,
}

impl<'src> SemCheckServer<'src> {
    pub fn new() -> (SemCheckServer<'src>, SemCheckContext<'src>) {
        // 各種意味チェッカの初期化
        let (var_decl_checker, var_decl_ctx) = VarDeclChecker::new();

        // サーバ, コンテキストの準備
        let server = SemCheckServer {
            var_decl: var_decl_checker,
        };
        let context = SemCheckContext {
            name: Name::new::<64>(),
            var_decl: var_decl_ctx,
        };

        (server, context)
    }
}
