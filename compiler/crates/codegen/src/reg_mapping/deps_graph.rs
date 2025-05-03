use std::collections::{HashMap, HashSet};

use sb_compiler_lirgen_ir::{LirTree, Call};

pub fn build_deps_graph(lir_tree: &LirTree) -> HashMap<u32, Vec<u32>> {
    DepsGraphBuilder::build(lir_tree)
}

struct DepsGraphBuilder {
    begin_point: Vec<u32>,
    end_point: Vec<(u32, u32)>,
}

impl DepsGraphBuilder {
    fn build(lir_tree: &LirTree) -> HashMap<u32, Vec<u32>> {
        // 1. 寿命解析
        let mut builder = DepsGraphBuilder {
            begin_point: vec![0],
            end_point: vec![],
        };
        builder.find_begin_point(lir_tree);
        builder.find_end_point(lir_tree);
        builder.end_point.push((0, 0));

        // 2. 各タイミングで登場・消失するレジスタを追うイテレータを準備
        let begin_end_zip_iter = builder
            .begin_point
            .iter()
            .zip(builder.end_point.iter().rev());

        // 3. レジスタの依存関係をグラフとして構築
        let mut alive_regs = HashSet::new();
        let mut deps_graph = HashMap::new();
        for (begin, (end1, end2)) in begin_end_zip_iter {
            // 3-1. 登場・消失処理
            if *begin != 0 {
                alive_regs.insert(*begin);
                deps_graph.insert(*begin, vec![]);
            }
            if *end1 != 0 {
                alive_regs.remove(&end1);
            }
            if *end2 != 0 {
                alive_regs.remove(&end2);
            }

            // 3-2. 生存中のレジスタの依存関係をグラフに追加
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

    fn find_begin_point(&mut self, lir_tree: &LirTree) {
        match lir_tree {
            LirTree::Node { lirs, .. } => {
                for lir_tree in lirs {
                    self.find_begin_point(lir_tree);
                }
            }
            LirTree::Inst { dst, .. } => {
                self.begin_point.push(*dst)
            }
            _ => {}
        }
    }

    fn find_end_point(&mut self, lir_tree: &LirTree) {
        match lir_tree {
            LirTree::Node { lirs, .. } => {
                for lir_tree in lirs.iter().rev() {
                    self.find_end_point(lir_tree);
                }
            }
            LirTree::Inst { inst: Call(_, reg_args), .. } => {
                for reg_arg in reg_args {
                    self.end_point.push((*reg_arg as u32, 0));
                }
            }
            LirTree::Inst { src1, src2, .. } => {
                self.end_point.push((*src1, *src2));
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use sb_compiler_parse::parse;
    use sb_compiler_lirgen::lirgen as lirgen0;

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
        let deps_graph = build_deps_graph0(&lir);
        deps_graph
    }
}
