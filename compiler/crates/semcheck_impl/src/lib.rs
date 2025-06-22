mod name;

use std::sync::{Arc, Mutex};

use sb_compiler_semcheck_impl_vardecl::{VarDeclChecker, VarDeclContext};
use sb_compiler_semcheck_impl_typedecl::{TypeDeclChecker, TypeDeclContext};

use name::Name;

#[derive(Clone)]
pub struct SemCheckContext<'src> {
    pub name: Name,
    pub var_decl: VarDeclContext<'src>,
    pub type_decl: TypeDeclContext,
}

#[allow(dead_code)]
pub struct SemCheckServer<'src> {
    var_decl: Arc<Mutex<VarDeclChecker<'src>>>,
    type_decl: Arc<Mutex<TypeDeclChecker>>,
}

impl<'src> SemCheckServer<'src> {
    pub fn new() -> (SemCheckServer<'src>, SemCheckContext<'src>) {
        // 各種意味チェッカの初期化
        let (var_decl_checker, var_decl_ctx) = VarDeclChecker::new();
        let (type_decl_checker, type_decl_ctx) = TypeDeclChecker::new();

        // サーバ, コンテキストの準備
        let server = SemCheckServer {
            var_decl: var_decl_checker,
            type_decl: type_decl_checker,
        };
        let context = SemCheckContext {
            name: Name::new::<64>(),
            var_decl: var_decl_ctx,
            type_decl: type_decl_ctx,
        };

        (server, context)
    }
}
