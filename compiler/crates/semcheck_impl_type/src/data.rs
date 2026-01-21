use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use lctree::{FindMax, LinkCutTree};
use string_interner::backend::StringBackend;
use string_interner::symbol::SymbolU32;
use string_interner::StringInterner;

use sb_compiler_semcheck_async::block_on;
use sb_compiler_type::r#type::*;
use sb_compiler_type::Typed;

use crate::func::{TypeContext, ty_register};

pub struct TypeTree {
    pub(crate) interner: StringInterner<StringBackend>,
    pub(crate) types: HashMap<SymbolU32, Arc<Type>>,
    pub(crate) tree: LinkCutTree<FindMax>,
    pub(crate) root_node: usize,
    pub(crate) nodes: HashMap<SymbolU32, usize>,
}

impl TypeTree {
    pub fn new() -> Arc<Mutex<TypeTree>> {
        // インターン化環境 (root: `.`)
        let interner = StringInterner::default();

        // 型参照木
        let mut tree = LinkCutTree::default();
        let root_node = tree.make_tree(0.);

        // TypeTree 初期化
        let tree = Arc::new(Mutex::new(TypeTree {
            interner,
            types: HashMap::new(),
            tree,
            root_node,
            nodes: HashMap::new(),
        }));

        // プリミティブ型の登録
        let mut ctx = TypeContext::from(Arc::clone(&tree));
        block_on(async {
            ty_register(&mut ctx, "bool", Bool.ty()).await.unwrap();
            ty_register(&mut ctx, "char", Char.ty()).await.unwrap();
            ty_register(&mut ctx, "i8", I8.ty()).await.unwrap();
            ty_register(&mut ctx, "i16", I16.ty()).await.unwrap();
            ty_register(&mut ctx, "i32", I32.ty()).await.unwrap();
        });
        tree.lock().unwrap().root_node = ctx.current_pos;

        tree
    }
}
