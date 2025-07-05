mod error;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use petgraph::algo::astar;
use petgraph::graph::{Graph, NodeIndex};

use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_semcheck_async::prelude::*;
use sb_compiler_semcheck_async_macros::failable_as_async;
use sb_compiler_type::r#type::*;
use sb_compiler_type::Typed;

use error::VarDeclError;

#[derive(Debug, Clone, Hash)]
pub struct Var<'src> {
    pub id: NodeIndex,
    pub span: Span<'src>,
    pub ty: Arc<Type>,
}

impl PartialEq for Var<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Var<'_> {}

impl<'src> Spanned<'src> for Var<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}

impl Typed for Var<'_> {
    fn ty(&self) -> Arc<Type> {
        self.ty.ty()
    }
}

#[derive(Debug, Clone)]
pub struct VarDeclContext<'src> {
    checker: Arc<Mutex<VarDeclChecker<'src>>>,
    node: NodeIndex,
}

#[derive(Debug)]
pub struct VarDeclChecker<'src> {
    graph: Graph<Var<'src>, ()>,
    root_node: NodeIndex,
    nodes: HashMap<Var<'src>, NodeIndex>,
}

impl<'src> VarDeclChecker<'src> {
    pub fn new() -> (Arc<Mutex<Self>>, VarDeclContext<'src>) {
        // ルート用の疑似変数
        let root_span = Span {
            src: ".",
            body: (0, 1),
            full: (0, 1),
        };
        let root_var = Var {
            id: NodeIndex::new(0),
            span: root_span,
            ty: Arc::new(I32),
        };

        // 変数参照グラフ
        let mut graph = Graph::new();
        let root_node = graph.add_node(root_var.clone());
        graph.node_weight_mut(root_node).unwrap().id = root_node;

        // チェッカ, コンテキスト (複数の async 文脈で共有するために Arc, Mutex でラップ)
        let checker = Arc::new(Mutex::new(VarDeclChecker {
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

    pub fn register(ctx: &mut VarDeclContext<'src>, span: &Span<'src>, ty: Arc<Type>) -> miette::Result<Var<'src>> {
        let mut checker = ctx.checker.lock().unwrap();

        // 1. 変数名を登録
        let mut var = Var {
            id: NodeIndex::new(0),
            span: *span,
            ty,
        };

        // 2. 参照グラフにノードを追加
        let from = checker.graph.add_node(var.clone());
        var.id = from;
        *checker.graph.node_weight_mut(from).unwrap() = var.clone();

        // 3. あああ
        let to = ctx.node;
        checker.graph.add_edge(from, to, ());
        checker.nodes.insert(var.clone(), from);
        ctx.node = from;

        Ok(var)
    }

    #[failable_as_async('a, 'src)]
    pub fn find<'a>(ctx: &'a VarDeclContext<'src>, span: &'a Span<'src>) -> miette::Result<Var<'src>> {
        let checker = ctx.checker.lock().unwrap();

        // 1. 可視変数の洗い出し
        let from = ctx.node;
        let to = checker.root_node;
        let waypoints = astar(
            &checker.graph,
            from,
            |n| n == to,
            |_| 0,      // 連結を確認するだけなので辺の重みは無視
            |_| 0,      // 連結を確認するだけなので辺の重みは無視
        ).unwrap().1;

        // 2. 経路を順に見て，初めて見つけた変数を検索結果とする
        for waypoint in waypoints {
            let waypoint_var = checker.graph.node_weight(waypoint).unwrap();
            if span.as_str() == waypoint_var.span.as_str() {
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
                Arc::new(I32),
            ).unwrap();

            // . <- var_a <- [here]
            let mut ctx_1 = ctx.clone();
            {
                // . <- var_a <- var_b
                let _ = VarDeclChecker::register(
                    &mut ctx_1,
                    &span("var_b"),
                    Arc::new(I32),
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
                    Arc::new(I32),
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
                Arc::new(I32),
            ).unwrap();

            // . <- var_a_0 <- var_a_1
            let var_a_1 = VarDeclChecker::register(
                &mut ctx,
                &span("var_a"),
                Arc::new(I32),
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
