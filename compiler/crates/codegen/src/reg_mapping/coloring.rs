use std::collections::HashMap;

pub fn coloring(deps_graph: HashMap<u32, Vec<u32>>) -> HashMap<u32, u8> {
    // 1. 接続頂点数の多い順にソート
    let mut connections = Vec::with_capacity(deps_graph.len());
    for (node, edges) in deps_graph.iter() {
        connections.push((node, edges.len()));
    }
    connections.sort_by(|a, b| b.1.cmp(&a.1));

    // 2. 結果用変数用意 (r0 ~ r19 は確保済みなので固定値)
    let mut result = HashMap::from([
        (0, 0), (1, 1), (2, 2), (3, 3), (4, 4),
        (5, 5), (6, 6), (7, 7), (8, 8), (9, 9),
        (10, 10), (11, 11), (12, 12), (13, 13), (14, 14),
        (15, 15), (16, 16), (17, 17), (18, 18), (19, 19),
    ]);

    // 2. グラフ彩色問題を解く (Welsh-Powell法)
    for (node, _) in connections {
        // 2-1. 彩色済みかどうか確認
        if result.contains_key(node) {
            continue;
        }

        // 2-2. 始点となる頂点に接続されている頂点を取得
        let dst_nodes = deps_graph.get(node).unwrap();

        // 2-3. 彩色対象となる各色について，彩色可能かどうかを判定
        let mut available = None;
        'outer: for color in 20..=29 {
            for dst_node in dst_nodes {
                if let Some(&dst_color) = result.get(dst_node) {
                    if dst_color == color {
                        continue 'outer;
                    }
                }
            }
            available = Some(color);
            break;
        }

        // 2-4. 彩色可能な色があれば，その色を割り当てる
        match available {
            Some(color) => result.insert(*node, color),
            None => panic!("failed to coloring"),
        };
    }

    result
}

#[cfg(test)]
mod tests {
    use super::coloring;

    #[test]
    fn test_coloring_1() {
        let deps_graph = vec![
            (1, vec![]),
            (2, vec![3, 4]),
            (3, vec![2]),
            (4, vec![2, 5]),
            (5, vec![4, 6]),
            (6, vec![5]),
            (7, vec![]),
        ]
        .into_iter()
        .collect();

        let result = coloring(deps_graph);
        assert_eq!(result[&1], 20);
        assert!(result[&2] != result[&3] && result[&2] != result[&4]);
        assert!(result[&4] != result[&5]);
        assert!(result[&5] != result[&6]);
    }

    #[test]
    fn test_coloring_2() {
        let deps_graph = vec![
            (1, vec![2, 3]),
            (2, vec![1, 3]),
            (3, vec![1, 2]),
            (4, vec![5]),
            (5, vec![4]),
            (6, vec![7]),
            (7, vec![6]),
        ]
        .into_iter()
        .collect();

        let result = coloring(deps_graph);
        assert!(result[&1] != result[&2] && result[&1] != result[&3]);
        assert!(result[&4] != result[&5]);
        assert!(result[&6] != result[&7]);
    }
}
