use std::collections::{HashMap, HashSet};

pub fn build_deps_graph(
    lifetime_tracker: impl Iterator<Item = (u32, Vec<u32>)>,
) -> HashMap<u32, Vec<u32>> {
    let mut alive_regs = HashSet::new();
    let mut deps_graph = HashMap::new();
    for (begin, ends) in lifetime_tracker {
        // 1. 登場・消失処理 (r0 ~ r19 は確保済みなので対象から除外)
        if begin > 19 {
            alive_regs.insert(begin);
            if deps_graph.get(&begin).is_none() {
                deps_graph.insert(begin, vec![]);
            }
        }
        for end in ends.into_iter().filter(|&x| x > 19) {
            alive_regs.remove(&end);
        }

        // 2. 生存中のレジスタの依存関係をグラフに追加
        for edge_src in &alive_regs {
            for edge_dst in &alive_regs {
                if edge_src != edge_dst {
                    deps_graph.get_mut(edge_src).unwrap().push(*edge_dst);
                }
            }
        }
    }

    deps_graph
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use sb_compiler_parse::parse;
    use sb_compiler_lirgen::lirgen as lirgen0;

    use crate::reg_mapping::lifetime::analyze_lifetime;
    use super::build_deps_graph as build_deps_graph0;

    #[test]
    fn test_reg_deps_graph_1() {
        const PROGRAM: &str = r#"
            var a: i32 = 10;
            var b: i32 = 20;
            var c: i32 = a + b;
        "#;
        // ===== Lir =====
        // li   t1 = 10
        // add  t2 = t0 + t1
        // li   t3 = 20
        // add  t4 = t0 + t3
        // add  t5 = t0 + t2
        // add  t6 = t0 + t4
        // add  t7 = t5 + t6
        // add  t8 = t0 + t7

        let deps_graph = build_deps_graph(PROGRAM);

        assert_eq!(deps_graph.get(&1), Some(&vec![]));
        assert_eq!(deps_graph.get(&2), Some(&vec![3, 4]));
        assert_eq!(deps_graph.get(&3), Some(&vec![2]));
        assert_eq!(deps_graph.get(&4), Some(&vec![2, 5]));
        assert_eq!(deps_graph.get(&5), Some(&vec![4, 6]));
        assert_eq!(deps_graph.get(&6), Some(&vec![5]));
        assert_eq!(deps_graph.get(&7), Some(&vec![]));
    }

    #[test]
    fn test_reg_deps_graph_2() {
        const PROGRAM: &str = r#"
            var a: i32 = 10;
            var b: i32 = 20;
            var c: i32 = a land b;
        "#;
        // ===== Lir =====
        //     li   t1 = 10
        //     add  t2 = t0 + t1
        //     li   t3 = 20
        //     add  t4 = t0 + t3
        //     add  t5 = t0 + t2
        //     bne  t0, (t0 != t5) -> 12
        //     jmp  t0, @local.0
        //     add  t6 = t0 + t4
        //     bne  t0, (t0 != t5) -> 12
        //     jmp  t0, @local.0
        //     li   t7 = 1
        //     jmp  t0, @local.1
        // @local.0
        //     li   t7 = 0
        // @local.1
        //     add  t8 = t0 + t7

        let deps_graph = build_deps_graph(PROGRAM);

        assert_eq!(deps_graph.get(&1), Some(&vec![]));
        assert_eq!(deps_graph.get(&2), Some(&vec![3, 4]));
        assert_eq!(deps_graph.get(&3), Some(&vec![2]));
        assert_eq!(deps_graph.get(&4), Some(&vec![2, 5]));
        assert_eq!(deps_graph.get(&5), Some(&vec![4]));
        assert_eq!(deps_graph.get(&6), Some(&vec![]));
        assert_eq!(deps_graph.get(&7), Some(&vec![]));
        assert_eq!(deps_graph.get(&8), Some(&vec![]));
    }

    #[test]
    fn test_reg_deps_graph_3() {
        const PROGRAM: &str = r#"
            var a: i32 = 10;
            var b: i32 = 20;
            var c: i32 = test_func(a, b);
        "#;
        // ===== Lir =====
        // li   t1 = 10
        // add  t2 = t0 + t1
        // li   t3 = 20
        // add  t4 = t0 + t3
        // add  t5 = t0 + t2
        // add  t6 = t0 + t4
        // call test_func.global (t5, t6)
        // add  t7 = t0 + t6

        let deps_graph = build_deps_graph(PROGRAM);

        assert_eq!(deps_graph.get(&1), Some(&vec![]));
        assert_eq!(deps_graph.get(&2), Some(&vec![3, 4]));
        assert_eq!(deps_graph.get(&3), Some(&vec![2]));
        assert_eq!(deps_graph.get(&4), Some(&vec![2, 5]));
        assert_eq!(deps_graph.get(&5), Some(&vec![4, 6]));
        assert_eq!(deps_graph.get(&6), Some(&vec![5]));
        assert_eq!(deps_graph.get(&7), Some(&vec![]));
    }

    fn build_deps_graph(input: &str) -> HashMap<u32, Vec<u32>> {
        let ast = parse(input).unwrap();
        let lir = lirgen0(&ast);
        let lifetime_tracker = analyze_lifetime(&lir);
        let deps_graph = build_deps_graph0(lifetime_tracker);
        deps_graph
    }
}
