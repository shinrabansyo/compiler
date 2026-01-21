mod var_register;
mod var_find;

use std::sync::{Arc, Mutex};

use petgraph::graph::NodeIndex;

use crate::data::VarGraph;

pub use var_register::var_register;
pub use var_find::var_find;

#[derive(Debug, Clone)]
pub struct VarContext<'src> {
    graph: Arc<Mutex<VarGraph<'src>>>,
    node: NodeIndex,
}

impl<'src> From<Arc<Mutex<VarGraph<'src>>>> for VarContext<'src> {
    fn from(graph: Arc<Mutex<VarGraph<'src>>>) -> Self {
        let node = graph.lock().unwrap().root_node;
        VarContext { graph, node }
    }
}

// #[cfg(test)]
// mod tests {
//     use std::sync::Arc;

//     use sb_compiler_parse_cst::Span;
//     use sb_compiler_semcheck_async::block_on;
//     use sb_compiler_type::r#type::*;

//     use super::VarDeclChecker;

//     #[test]
//     fn test_decl_and_find() {
//         let (_, mut ctx) = VarDeclChecker::new();

//         block_on(async {
//             // . <- var_a
//             let _ = VarDeclChecker::register(
//                 &mut ctx,
//                 &span("var_a"),
//                 Arc::new(I32),
//             ).unwrap();

//             // . <- var_a <- [here]
//             let mut ctx_1 = ctx.clone();
//             {
//                 // . <- var_a <- var_b
//                 let _ = VarDeclChecker::register(
//                     &mut ctx_1,
//                     &span("var_b"),
//                     Arc::new(I32),
//                 ).unwrap();

//                 // . <- var_a <- var_b <- [here]
//                 assert!(VarDeclChecker::find(&ctx_1, &span("var_a")).await.is_ok());
//                 assert!(VarDeclChecker::find(&ctx_1, &span("var_b")).await.is_ok());
//                 assert!(VarDeclChecker::find(&ctx_1, &span("var_c")).await.is_err());
//             }

//             // . <- var_a <- [here]
//             let mut ctx_2 = ctx.clone();
//             {
//                 // . <- var_a <- var_c
//                 let _ = VarDeclChecker::register(
//                     &mut ctx_2,
//                     &span("var_c"),
//                     Arc::new(I32),
//                 ).unwrap();

//                 // . <- var_a <- var_c <- [here]
//                 assert!(VarDeclChecker::find(&ctx_2, &span("var_a")).await.is_ok());
//                 assert!(VarDeclChecker::find(&ctx_2, &span("var_b")).await.is_err());
//                 assert!(VarDeclChecker::find(&ctx_2, &span("var_c")).await.is_ok());
//             }
//         });
//     }

//     #[test]
//     fn test_shadowing() {
//         let (_, mut ctx) = VarDeclChecker::new();

//         block_on(async {
//             // . <- var_a_0
//             let var_a_0 = VarDeclChecker::register(
//                 &mut ctx,
//                 &span("var_a"),
//                 Arc::new(I32),
//             ).unwrap();

//             // . <- var_a_0 <- var_a_1
//             let var_a_1 = VarDeclChecker::register(
//                 &mut ctx,
//                 &span("var_a"),
//                 Arc::new(I32),
//             ).unwrap();

//             // . <- var_a_0 <- var_a_1 <- [here]
//             assert_ne!(var_a_0, var_a_1);
//             assert_ne!(var_a_0, block_on(VarDeclChecker::find(&ctx, &span("var_a"))).unwrap());
//             assert_eq!(var_a_1, block_on(VarDeclChecker::find(&ctx, &span("var_a"))).unwrap());
//         });
//     }

//     fn span(s: &str) -> Span {
//         Span {
//             src: s,
//             body: (0, s.len()),
//             full: (0, s.len()),
//         }
//     }
// }
