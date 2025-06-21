mod error;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use petgraph::algo::astar;
use petgraph::graph::{Graph, NodeIndex};
use string_interner::backend::StringBackend;
use string_interner::symbol::SymbolU32;
use string_interner::StringInterner;

use sb_compiler_semcheck_async::prelude::*;
use sb_compiler_semcheck_async_macros::failable_as_async;

use error::VarDeclError;

pub type VarId = NodeIndex;

#[derive(Debug, Clone)]
pub struct VarDeclContext {
    checker: Arc<Mutex<VarDeclChecker>>,
    node: NodeIndex,
}

#[derive(Debug)]
pub struct VarDeclChecker {
    interner: StringInterner<StringBackend>,
    graph: Graph<SymbolU32, ()>,
    root_node: NodeIndex,
    nodes: HashMap<SymbolU32, NodeIndex>,
}

impl VarDeclChecker {
    pub fn new() -> (Arc<Mutex<Self>>, VarDeclContext) {
        // インターン化環境 (root: `.`)
        let mut interner = StringInterner::default();
        let root_symbol = interner.get_or_intern(".");

        // 変数参照グラフ
        let mut graph = Graph::new();
        let root_node = graph.add_node(root_symbol);

        // チェッカ, コンテキスト (複数の async 文脈で共有するために Arc, Mutex でラップ)
        let checker = Arc::new(Mutex::new(VarDeclChecker {
            interner,
            graph,
            root_node,
            nodes: HashMap::from([(root_symbol, root_node)]),
        }));
        let context = VarDeclContext {
            checker: Arc::clone(&checker),
            node: root_node,
        };

        (checker, context)
    }

    pub fn register(ctx: &mut VarDeclContext, name: &str) -> anyhow::Result<NodeIndex> {
        let mut checker = ctx.checker.lock().unwrap();

        // 1. 変数名を登録
        let var_symbol = checker.interner.get_or_intern(name);

        // 2. 参照グラフに追加
        let from = checker.graph.add_node(var_symbol);
        let to = ctx.node;
        checker.graph.add_edge(from, to, ());
        checker.nodes.insert(var_symbol, from);

        // 3. 以降の文脈で最新の変数を参照できるように更新
        ctx.node = from;

        Ok(from)
    }

    pub fn exists<'a>(ctx: &'a VarDeclContext, name: &'a str) -> Option<NodeIndex> {
        let checker = ctx.checker.lock().unwrap();

        // 1. 変数名を検索
        let var_symbol = match checker.interner.get(name) {
            Some(symbol) => symbol,
            None => return None,
        };

        // 2. 可視変数の洗い出し
        let from = ctx.node;
        let to = checker.root_node;
        let waypoints = astar(
            &checker.graph,
            from,
            |n| n == to,
            |_| 0,      // 連結を確認するだけなので辺の重みは無視
            |_| 0,      // 連結を確認するだけなので辺の重みは無視
        ).unwrap().1;

        // 3. 経路を順に見て，初めて見つけた変数を検索結果とする
        for waypoint in waypoints {
            if &var_symbol == checker.graph.node_weight(waypoint).unwrap() {
                return Some(waypoint);
            }
        }

        None
    }

    #[failable_as_async('a)]
    pub fn find<'a>(ctx: &'a VarDeclContext, name: &'a str) -> anyhow::Result<NodeIndex> {
        VarDeclChecker::exists(ctx, name).ok_or(
            VarDeclError::new_not_declared(name.to_string())
        )
    }
}

#[cfg(test)]
mod tests {
    use sb_compiler_semcheck_async::block_on;

    use super::VarDeclChecker;

    #[test]
    fn test_decl_and_find() {
        let (_, mut ctx) = VarDeclChecker::new();

        block_on(async {
            // . <- var_a
            let _ = VarDeclChecker::register(&mut ctx, "var_a").unwrap();

            // . <- var_a <- [here]
            let mut ctx_1 = ctx.clone();
            {
                // . <- var_a <- var_b
                let _ = VarDeclChecker::register(&mut ctx_1, "var_b").unwrap();

                // . <- var_a <- var_b <- [here]
                assert!(VarDeclChecker::find(&ctx_1, "var_a").await.is_ok());
                assert!(VarDeclChecker::find(&ctx_1, "var_b").await.is_ok());
                assert!(VarDeclChecker::find(&ctx_1, "var_c").await.is_err());
            }

            // . <- var_a <- [here]
            let mut ctx_2 = ctx.clone();
            {
                // . <- var_a <- var_c
                let _ = VarDeclChecker::register(&mut ctx_2, "var_c").unwrap();

                // . <- var_a <- var_c <- [here]
                assert!(VarDeclChecker::find(&ctx_2, "var_a").await.is_ok());
                assert!(VarDeclChecker::find(&ctx_2, "var_b").await.is_err());
                assert!(VarDeclChecker::find(&ctx_2, "var_c").await.is_ok());
            }
        });
    }

    #[test]
    fn test_shadowing() {
        let (_, mut ctx) = VarDeclChecker::new();

        block_on(async {
            // . <- var_a_0
            let var_a_0 = VarDeclChecker::register(&mut ctx, "var_a").unwrap();

            // . <- var_a_0 <- var_a_1
            let var_a_1 = VarDeclChecker::register(&mut ctx, "var_a").unwrap();

            // . <- var_a_0 <- var_a_1 <- [here]
            assert_ne!(var_a_0, var_a_1);
            assert_ne!(var_a_0, block_on(VarDeclChecker::find(&ctx, "var_a")).unwrap());
            assert_eq!(var_a_1, block_on(VarDeclChecker::find(&ctx, "var_a")).unwrap());
        });
    }
}
