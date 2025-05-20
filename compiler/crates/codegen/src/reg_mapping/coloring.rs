use std::collections::{HashMap, HashSet};

pub fn coloring(
    deps_graph: HashMap<u32, HashSet<u32>>,
    usable_regs: &[u8],
) -> HashMap<u32, u8> {
    // 1. 接続頂点数の多い順にソート
    let mut connections = Vec::with_capacity(deps_graph.len());
    for (node, edges) in deps_graph.iter() {
        connections.push((node, edges.len()));
    }
    connections.sort_by(|a, b| b.1.cmp(&a.1));

    // 2. グラフ彩色問題を解く (Welsh-Powell法)
    let mut result = HashMap::<u32, u8>::new();
    for (node, _) in connections {
        // 2-1. 彩色済みかどうか確認
        if result.contains_key(node) {
            continue;
        }

        // 2-2. 始点となる頂点に接続されている頂点を取得
        let dst_nodes = deps_graph.get(node).unwrap();

        // 2-3. 彩色対象となる各色について，彩色可能かどうかを判定
        let mut available = None;
        'outer: for color in usable_regs {
            for dst_node in dst_nodes {
                if let Some(&dst_color) = result.get(dst_node) {
                    if dst_color == *color {
                        continue 'outer;
                    }
                }
            }
            available = Some(color);
            break;
        }

        // 2-4. 彩色可能な色があれば，その色を割り当てる
        match available {
            Some(color) => result.insert(*node, *color),
            None => panic!("failed to coloring"),
        };
    }

    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::coloring;

    const USABLE_REGS: [u8; 10] = [20, 21, 22, 23, 24, 25, 26, 27, 28, 29];

    #[test]
    fn test_coloring_1() {
        let deps_graph = vec![
            (20, HashSet::from([])),
            (21, HashSet::from([22, 23])),
            (22, HashSet::from([21])),
            (23, HashSet::from([21, 24])),
            (24, HashSet::from([23, 25])),
            (25, HashSet::from([24])),
            (26, HashSet::from([])),
        ]
        .into_iter()
        .collect();

        let result = coloring(deps_graph, &USABLE_REGS);
        assert_eq!(result[&20], 20);
        assert_eq!(result[&26], 20);
        assert!(result[&21] != result[&22] && result[&21] != result[&23]);
        assert!(result[&23] != result[&24]);
        assert!(result[&24] != result[&25]);
    }

    #[test]
    fn test_coloring_2() {
        let deps_graph = vec![
            (20, HashSet::from([21, 22])),
            (21, HashSet::from([20, 22])),
            (22, HashSet::from([20, 21])),
            (23, HashSet::from([24])),
            (24, HashSet::from([23])),
            (25, HashSet::from([26])),
            (26, HashSet::from([25])),
        ]
        .into_iter()
        .collect();

        let result = coloring(deps_graph, &USABLE_REGS);
        assert!(result[&20] != result[&21] && result[&20] != result[&22]);
        assert!(result[&23] != result[&24]);
        assert!(result[&25] != result[&26]);
    }
}
