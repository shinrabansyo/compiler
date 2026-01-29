pub mod decl;
pub mod r#fn;
pub mod infer;
pub mod op;
pub mod parse;

use std::sync::{Arc, Mutex};

use sb_compiler_semcheck_async::block_on;
use sb_compiler_utils::collections::{LayeredGraph, LayeredGraphCursor};

use crate::r#type::{Typed, Type, Bool, Char, I8, I16, I32};

use decl::ty_register;

#[derive(Debug, Clone)]
pub struct TypeContext<'src> {
    pub(crate) graph: Arc<Mutex<LayeredGraph<(&'src str, Arc<Type>)>>>,
    pub(crate) cur: LayeredGraphCursor,
}

impl<'src> TypeContext<'src> {
    pub fn new() -> Self {
        // 型空間の初期化
        let (graph, cur) = LayeredGraph::new_with_undirected();
        let mut ctx = TypeContext {
            graph: Arc::new(Mutex::new(graph)),
            cur,
        };

        // プリミティブ型の登録
        block_on(async {
            ty_register(&mut ctx, "bool".into(), Bool.ty()).await.unwrap();
            ty_register(&mut ctx, "char".into(), Char.ty()).await.unwrap();
            ty_register(&mut ctx, "i8".into(), I8.ty()).await.unwrap();
            ty_register(&mut ctx, "i16".into(), I16.ty()).await.unwrap();
            ty_register(&mut ctx, "i32".into(), I32.ty()).await.unwrap();
        });

        ctx
    }
}
