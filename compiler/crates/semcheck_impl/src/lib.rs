use std::sync::{Arc, Mutex};

use sb_compiler_semcheck_impl_var::VarContext;
use sb_compiler_semcheck_impl_type::{TypeContext, TypeTree};
use sb_compiler_utils::str::PartedString;

#[derive(Clone)]
pub struct SemCheckContext<'src> {
    pub name: PartedString,
    pub var: VarContext<'src>,
    pub r#type: TypeContext,
}

#[allow(dead_code)]
pub struct SemCheckDataStore {
    type_tree: Arc<Mutex<TypeTree>>,
}

impl SemCheckDataStore {
    pub fn new<'src>() -> (SemCheckContext<'src>, SemCheckDataStore) {
        // 各種意味チェッカの初期化
        let type_tree = TypeTree::new();

        // 意味解析用コンテキスト，データの準備
        let ctx = SemCheckContext {
            name: PartedString::new::<64>(),
            var: VarContext::new(),
            r#type: TypeContext::from(Arc::clone(&type_tree)),
        };
        let server = SemCheckDataStore { type_tree };

        (ctx, server)
    }
}
