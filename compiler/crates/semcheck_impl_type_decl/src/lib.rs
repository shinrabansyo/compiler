mod error;
pub mod r#type;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use lctree::{FindMax, LinkCutTree};
use string_interner::backend::StringBackend;
use string_interner::symbol::SymbolU32;
use string_interner::StringInterner;

use sb_compiler_semcheck_async::prelude::*;
use sb_compiler_semcheck_async_macros::failable_as_async;

use error::TypeDeclError;
pub use r#type::Type;

#[derive(Clone)]
pub struct TypeDeclContext {
    checker: Arc<Mutex<TypeDeclChecker>>,
    current_pos: usize,
}

pub struct TypeDeclChecker {
    interner: StringInterner<StringBackend>,
    types: HashMap<SymbolU32, r#type::Type>,
    tree: LinkCutTree<FindMax>,
    nodes: HashMap<SymbolU32, usize>,
}

impl TypeDeclChecker {
    pub fn new() -> (Arc<Mutex<Self>>, TypeDeclContext) {
        // インターン化環境 (root: `.`)
        let interner = StringInterner::default();

        // 型参照木
        let mut tree = LinkCutTree::default();
        let root_node = tree.make_tree(0.);

        // チェッカ, コンテキスト (複数の async 文脈で共有するために Arc, Mutex でラップ)
        let checker = Arc::new(Mutex::new(TypeDeclChecker {
            interner,
            tree,
            types: HashMap::new(),
            nodes: HashMap::new(),
        }));
        let mut context = TypeDeclContext {
            checker: Arc::clone(&checker),
            current_pos: root_node,
        };

        // プリミティブ型の登録
        {
            use r#type::primitive::*;
            use r#type::Primitive;

            TypeDeclChecker::register(&mut context, "i8", Primitive(I8)).unwrap();
            TypeDeclChecker::register(&mut context, "i32", Primitive(I32)).unwrap();
        }

        (checker, context)
    }

    pub fn register(ctx: &mut TypeDeclContext, name: &str, ty: Type) -> anyhow::Result<()> {
        let mut checker = ctx.checker.lock().unwrap();

        // 1. 型名を登録
        let symbol = match checker.interner.get(name) {
            Some(_) => {
                let err = TypeDeclError::new_already_declared(name.to_string());
                return Err(err.into());
            }
            None => checker.interner.get_or_intern(name),
        };
        checker.types.insert(symbol, ty);

        // 2. 参照木に追加
        let parent = ctx.current_pos;
        let child = checker.tree.make_tree(0.);
        checker.tree.link(parent, child);
        checker.nodes.insert(symbol, child);

        // 3. 以降の文脈で新しい型を参照できるように更新
        ctx.current_pos = child;

        Ok(())
    }

    #[failable_as_async('a)]
    pub fn find<'a>(ctx: &'a TypeDeclContext, name: &'a str) -> anyhow::Result<Type> {
        let mut checker = ctx.checker.lock().unwrap();

        // 1. 型名を検索
        let symbol = match checker.interner.get(name) {
            Some(symbol) => symbol,
            None => {
                let err = TypeDeclError::new_not_declared(name.to_string());
                return Err(err.into());
            }
        };

        // 2. 型が可視であるか確認
        let node = *checker.nodes.get(&symbol).unwrap();
        if checker.tree.connected(ctx.current_pos, node) {
            Ok(checker.types.get(&symbol).unwrap().clone())
        } else {
            let err = TypeDeclError::new_not_declared_in_scope(name.to_string());
            Err(err.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use sb_compiler_semcheck_async::block_on;

    use super::r#type::primitive::*;
    use super::r#type::*;
    use super::TypeDeclChecker;

    #[test]
    fn test_ok() {
        let (_, ctx) = TypeDeclChecker::new();

        block_on(async {
            assert_eq!(
                TypeDeclChecker::find(&ctx, "i32").await.unwrap(),
                Primitive(I32),
            );
        });
    }
}
