use std::collections::{HashMap, HashSet};

pub fn build_deps_graph<I>(lifetime_tracker: I) -> HashMap<u32, HashSet<u32>>
where
    I: Iterator<Item = (u32, Vec<u32>)>,
{
    let mut alive_regs = HashSet::new();
    let mut deps_graph = HashMap::new();
    for (begin, ends) in lifetime_tracker {
        // 1. 登場・消失処理 (t0 ~ t19 は直接実レジスタにマッピングするので無視)
        if begin > 19 {
            alive_regs.insert(begin);
            if deps_graph.get(&begin).is_none() {
                deps_graph.insert(begin, HashSet::new());
            }
        }
        for end in ends.into_iter().filter(|reg| *reg > 19) {
            alive_regs.remove(&end);
        }

        // 2. 生存中のレジスタの依存関係をグラフに追加
        for edge_src in &alive_regs {
            for edge_dst in &alive_regs {
                if edge_src != edge_dst {
                    deps_graph.get_mut(edge_src).unwrap().insert(*edge_dst);
                }
            }
        }
    }

    deps_graph
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use sb_compiler_lirgen_ir::{lir, LirBlock, Add, Ble, Jmp, Li};

    use crate::reg_mapping::lifetime::analyze_lifetime;
    use super::build_deps_graph;

    #[test]
    fn test_reg_deps_graph_1() {
        let lir = LirBlock::Single {
            result_reg: 0,
            lirs: vec![
                lir!(Li(10) 20),       // li t0 = 10
                lir!(Li(20) 21),       // li t1 = 20
                lir!(Add 22, 20, 21),  // add  t22 = t20 + t21
            ],
        };

        let lifetime_tracker = analyze_lifetime(&lir);
        let deps_graph = build_deps_graph(lifetime_tracker);

        assert_eq!(deps_graph.get(&20), Some(&HashSet::from([21])));
        assert_eq!(deps_graph.get(&21), Some(&HashSet::from([20])));
        assert_eq!(deps_graph.get(&22), Some(&HashSet::from([])));
    }

    #[test]
    fn test_reg_deps_graph_2() {
        let lir = LirBlock::Single {
            result_reg: 0,
            lirs: vec![
                lir!(Li(0) 20),      // li t20 = 0
                lir!(Add 21, 0, 20), // add t21 = t0 + t20 (cnt)
                lir!(Add 22, 0, 20), // add t22 = t0 + t20 (sum)
                lir!(Li(10) 23),     // li t23 = 10
                lir!(Add 24, 0, 23), // add t24 = t0 + t23
                LirBlock::Multiple {
                    lirs: vec![
                        lir!(Ble(12) 0, 24, 21), // ble r0, (t24 <= t21) -> 12
                        lir!(Li(1) 25),          // li t25 = 1
                        lir!(Add 21, 0, 25),     // addi t21 = t0 + t25
                        lir!(Add 22, 0, 25),     // addi t22 = t0 + t25
                        lir!(Jmp(-18)),          // jmp -18
                    ],
                },
            ],
        };

        let lifetime_tracker = analyze_lifetime(&lir);
        let deps_graph = build_deps_graph(lifetime_tracker);

        assert_eq!(deps_graph.get(&20), Some(&HashSet::from([21])));
        assert_eq!(deps_graph.get(&21), Some(&HashSet::from([20, 22, 23, 24, 25])));
        assert_eq!(deps_graph.get(&22), Some(&HashSet::from([21, 23, 24, 25])));
        assert_eq!(deps_graph.get(&23), Some(&HashSet::from([21, 22])));
        assert_eq!(deps_graph.get(&24), Some(&HashSet::from([21, 22, 25])));
        assert_eq!(deps_graph.get(&25), Some(&HashSet::from([21, 22, 24])));
    }
}
