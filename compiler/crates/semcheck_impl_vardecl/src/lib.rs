mod error;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use petgraph::algo::astar;
use petgraph::graph::{Graph, NodeIndex};
use string_interner::backend::StringBackend;
use string_interner::symbol::SymbolU32;
use string_interner::StringInterner;

use sb_compiler_parse_cst::Span;
use sb_compiler_semcheck_async::prelude::*;
use sb_compiler_semcheck_async_macros::failable_as_async;
use sb_compiler_type::r#type::*;

use error::VarDeclError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Var<'src> {
    pub symbol: SymbolU32,
    pub span: Span<'src>,
    pub ty: Arc<Type>,
}

#[derive(Debug, Clone)]
pub struct VarDeclContext<'src> {
    checker: Arc<Mutex<VarDeclChecker<'src>>>,
    node: NodeIndex,
}

#[derive(Debug)]
pub struct VarDeclChecker<'src> {
    interner: StringInterner<StringBackend>,
    graph: Graph<Var<'src>, ()>,
    root_node: NodeIndex,
    nodes: HashMap<Var<'src>, NodeIndex>,
}

impl<'src> VarDeclChecker<'src> {
    pub fn new() -> (Arc<Mutex<Self>>, VarDeclContext<'src>) {
        // インターン化環境 (root: `.`)
        let mut interner = StringInterner::default();
        let root_symbol = interner.get_or_intern(".");

        // ルート用の疑似変数
        let root_span = Span {
            src: ".",
            body: (0, 1),
            full: (0, 1),
        };
        let root_var = Var {
            symbol: root_symbol,
            span: root_span,
            ty: Arc::new(Primitive(I32)),
        };

        // 変数参照グラフ
        let mut graph = Graph::new();
        let root_node = graph.add_node(root_var.clone());

        // チェッカ, コンテキスト (複数の async 文脈で共有するために Arc, Mutex でラップ)
        let checker = Arc::new(Mutex::new(VarDeclChecker {
            interner,
            graph,
            root_node,
            nodes: HashMap::from([(root_var, root_node)]),
        }));
        let context = VarDeclContext {
            checker: Arc::clone(&checker),
            node: root_node,
        };

        (checker, context)
    }

    pub fn register(ctx: &mut VarDeclContext<'src>, span: &Span<'src>, ty: Arc<Type>) -> anyhow::Result<Var<'src>> {
        let mut checker = ctx.checker.lock().unwrap();

        // 1. 変数名を登録
        let var_symbol = checker.interner.get_or_intern(span.as_str());
        let var = Var {
            symbol: var_symbol,
            span: *span,
            ty,
        };

        // 2. 参照グラフに追加
        let from = checker.graph.add_node(var.clone());
        let to = ctx.node;
        checker.graph.add_edge(from, to, ());
        checker.nodes.insert(var.clone(), from);

        // 3. 以降の文脈で最新の変数を参照できるように更新
        ctx.node = from;

        Ok(var)
    }

    #[failable_as_async('a, 'src)]
    pub fn find<'a>(ctx: &'a VarDeclContext<'src>, span: &'a Span<'src>) -> anyhow::Result<Var<'src>> {
        let checker = ctx.checker.lock().unwrap();

        // 1. 変数名を検索
        let var_symbol = match checker.interner.get(span.as_str()) {
            Some(symbol) => symbol,
            None => return Err(VarDeclError::new_not_declared(*span).into()),
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
            let waypoint_var = checker.graph.node_weight(waypoint).unwrap();
            if var_symbol == waypoint_var.symbol {
                return Ok(waypoint_var.clone());
            }
        }

        Err(VarDeclError::new_not_declared(*span))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sb_compiler_parse_cst::Span;
    use sb_compiler_semcheck_async::block_on;
    use sb_compiler_type::r#type::*;

    use super::VarDeclChecker;

    #[test]
    fn test_decl_and_find() {
        let (_, mut ctx) = VarDeclChecker::new();

        block_on(async {
            // . <- var_a
            let _ = VarDeclChecker::register(
                &mut ctx,
                &span("var_a"),
                Arc::new(Primitive(I32)),
            ).unwrap();

            // . <- var_a <- [here]
            let mut ctx_1 = ctx.clone();
            {
                // . <- var_a <- var_b
                let _ = VarDeclChecker::register(
                    &mut ctx_1,
                    &span("var_b"),
                    Arc::new(Primitive(I32)),
                ).unwrap();

                // . <- var_a <- var_b <- [here]
                assert!(VarDeclChecker::find(&ctx_1, &span("var_a")).await.is_ok());
                assert!(VarDeclChecker::find(&ctx_1, &span("var_b")).await.is_ok());
                assert!(VarDeclChecker::find(&ctx_1, &span("var_c")).await.is_err());
            }

            // . <- var_a <- [here]
            let mut ctx_2 = ctx.clone();
            {
                // . <- var_a <- var_c
                let _ = VarDeclChecker::register(
                    &mut ctx_2,
                    &span("var_c"),
                    Arc::new(Primitive(I32)),
                ).unwrap();

                // . <- var_a <- var_c <- [here]
                assert!(VarDeclChecker::find(&ctx_2, &span("var_a")).await.is_ok());
                assert!(VarDeclChecker::find(&ctx_2, &span("var_b")).await.is_err());
                assert!(VarDeclChecker::find(&ctx_2, &span("var_c")).await.is_ok());
            }
        });
    }

    #[test]
    fn test_shadowing() {
        let (_, mut ctx) = VarDeclChecker::new();

        block_on(async {
            // . <- var_a_0
            let var_a_0 = VarDeclChecker::register(
                &mut ctx,
                &span("var_a"),
                Arc::new(Primitive(I32))
            ).unwrap();

            // . <- var_a_0 <- var_a_1
            let var_a_1 = VarDeclChecker::register(
                &mut ctx,
                &span("var_a"),
                Arc::new(Primitive(I32)),
            ).unwrap();

            // . <- var_a_0 <- var_a_1 <- [here]
            assert_ne!(var_a_0, var_a_1);
            assert_ne!(var_a_0, block_on(VarDeclChecker::find(&ctx, &span("var_a"))).unwrap());
            assert_eq!(var_a_1, block_on(VarDeclChecker::find(&ctx, &span("var_a"))).unwrap());
        });
    }

    fn span(s: &str) -> Span {
        Span {
            src: s,
            body: (0, s.len()),
            full: (0, s.len()),
        }
    }

}
