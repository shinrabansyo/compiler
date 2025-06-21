mod name;

use std::sync::{Arc, Mutex};

use sb_compiler_semcheck_impl_vardecl::{VarDeclChecker, VarDeclContext};

use name::Name;

#[derive(Debug, Clone)]
pub struct SemCheckContext<'input> {
    name: Name,
    pub var_decl: VarDeclContext<'input>,
}

impl<'input> SemCheckContext<'input> {
    pub fn push_namespace(&mut self, namespace: &str) {
        self.name.push(namespace);
    }

    pub fn pop_namespace(&mut self) {
        self.name.pop();
    }

    pub fn as_str_name(&mut self) -> &str {
        self.name.as_str()
    }

    pub fn as_str_name_with(&mut self, name: &str) -> &str {
        self.name.as_str_with(name)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct SemCheckServer<'input> {
    var_decl: Arc<Mutex<VarDeclChecker<'input>>>,
}

impl<'input> SemCheckServer<'input> {
    pub fn new() -> (SemCheckServer<'input>, SemCheckContext<'input>) {
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
