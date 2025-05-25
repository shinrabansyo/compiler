use std::collections::{HashMap, HashSet};

use super::{RegMap, MapTo};

pub fn coloring(deps_graph: HashMap<u32, HashSet<u32>>, usable_regs: &[u8]) -> RegMap {
    // 1. 接続頂点数の多い順にソート
    let mut connections = Vec::with_capacity(deps_graph.len());
    for (node, edges) in deps_graph.iter() {
        connections.push((node, edges.len()));
    }
    connections.sort_by(|a, b| b.1.cmp(&a.1));

    // 2. グラフ彩色問題を解く (Welsh-Powell法)
    let mut spilled_regs = 0;
    let mut result = HashMap::new();
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
                if let Some(&MapTo::Reg(dst_color)) = result.get(dst_node) {
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
            Some(color) => {
                result.insert(*node, MapTo::Reg(*color));
            }
            None => {
                result.insert(*node, MapTo::Stack(spilled_regs));
                spilled_regs += 1;
            }
        };
    }

    RegMap::from((spilled_regs, result))
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{coloring, MapTo};

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

        let reg_map = coloring(deps_graph, &USABLE_REGS);

        assert_eq!(reg_map.get(&20), MapTo::Reg(20));
        assert_eq!(reg_map.get(&26), MapTo::Reg(20));
        assert!(reg_map.get(&21) != reg_map.get(&22) && reg_map.get(&21) != reg_map.get(&23));
        assert!(reg_map.get(&23) != reg_map.get(&24));
        assert!(reg_map.get(&24) != reg_map.get(&25));
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

        let reg_map = coloring(deps_graph, &USABLE_REGS);

        assert!(reg_map.get(&20) != reg_map.get(&21) && reg_map.get(&20) != reg_map.get(&22));
        assert!(reg_map.get(&23) != reg_map.get(&24));
        assert!(reg_map.get(&25) != reg_map.get(&26));
    }

    #[test]
    fn test_coloring_3() {
        let deps_graph = vec![
            (20, HashSet::from([21, 22, 23, 24, 25, 26, 27, 28, 29, 30])),
            (21, HashSet::from([20, 22, 23, 24, 25, 26, 27, 28, 29, 30])),
            (22, HashSet::from([20, 21, 23, 24, 25, 26, 27, 28, 29, 30])),
            (23, HashSet::from([20, 21, 22, 24, 25, 26, 27, 28, 29, 30])),
            (24, HashSet::from([20, 21, 22, 23, 25, 26, 27, 28, 29, 30])),
            (25, HashSet::from([20, 21, 22, 23, 24, 26, 27, 28, 29, 30])),
            (26, HashSet::from([20, 21, 22, 23, 24, 25, 27, 28, 29, 30])),
            (27, HashSet::from([20, 21, 22, 23, 24, 25, 26, 28, 29, 30])),
            (28, HashSet::from([20, 21, 22, 23, 24, 25, 26, 27, 29, 30])),
            (29, HashSet::from([20, 21, 22, 23, 24, 25, 26, 27, 28, 30])),
            (30, HashSet::from([20, 21, 22, 23, 24, 25, 26, 27, 28, 29])),
        ]
        .into_iter()
        .collect();

        let reg_map = coloring(deps_graph, &USABLE_REGS);

        assert!(
            reg_map.get(&20) != reg_map.get(&21)
                && reg_map.get(&20) != reg_map.get(&22)
                && reg_map.get(&20) != reg_map.get(&23)
                && reg_map.get(&20) != reg_map.get(&24)
                && reg_map.get(&20) != reg_map.get(&25)
                && reg_map.get(&20) != reg_map.get(&26)
                && reg_map.get(&20) != reg_map.get(&27)
                && reg_map.get(&20) != reg_map.get(&28)
                && reg_map.get(&20) != reg_map.get(&29)
                && reg_map.get(&20) != reg_map.get(&30)
        );
        assert!(
            reg_map.get(&21) != reg_map.get(&22)
                && reg_map.get(&21) != reg_map.get(&23)
                && reg_map.get(&21) != reg_map.get(&24)
                && reg_map.get(&21) != reg_map.get(&25)
                && reg_map.get(&21) != reg_map.get(&26)
                && reg_map.get(&21) != reg_map.get(&27)
                && reg_map.get(&21) != reg_map.get(&28)
                && reg_map.get(&21) != reg_map.get(&29)
                && reg_map.get(&21) != reg_map.get(&30)
        );
        assert!(
            reg_map.get(&22) != reg_map.get(&23)
                && reg_map.get(&22) != reg_map.get(&24)
                && reg_map.get(&22) != reg_map.get(&25)
                && reg_map.get(&22) != reg_map.get(&26)
                && reg_map.get(&22) != reg_map.get(&27)
                && reg_map.get(&22) != reg_map.get(&28)
                && reg_map.get(&22) != reg_map.get(&29)
                && reg_map.get(&22) != reg_map.get(&30)
        );
        assert!(
            reg_map.get(&23) != reg_map.get(&24)
                && reg_map.get(&23) != reg_map.get(&25)
                && reg_map.get(&23) != reg_map.get(&26)
                && reg_map.get(&23) != reg_map.get(&27)
                && reg_map.get(&23) != reg_map.get(&28)
                && reg_map.get(&23) != reg_map.get(&29)
                && reg_map.get(&23) != reg_map.get(&30)
        );
        assert!(
            reg_map.get(&24) != reg_map.get(&25)
                && reg_map.get(&24) != reg_map.get(&26)
                && reg_map.get(&24) != reg_map.get(&27)
                && reg_map.get(&24) != reg_map.get(&28)
                && reg_map.get(&24) != reg_map.get(&29)
                && reg_map.get(&24) != reg_map.get(&30)
        );
        assert!(
            reg_map.get(&25) != reg_map.get(&26)
                && reg_map.get(&25) != reg_map.get(&27)
                && reg_map.get(&25) != reg_map.get(&28)
                && reg_map.get(&25) != reg_map.get(&29)
                && reg_map.get(&25) != reg_map.get(&30)
        );
        assert!(
            reg_map.get(&26) != reg_map.get(&27)
                && reg_map.get(&26) != reg_map.get(&28)
                && reg_map.get(&26) != reg_map.get(&29)
                && reg_map.get(&26) != reg_map.get(&30)
        );
        assert!(
            reg_map.get(&27) != reg_map.get(&28)
                && reg_map.get(&27) != reg_map.get(&29)
                && reg_map.get(&27) != reg_map.get(&30)
        );
        assert!(
            reg_map.get(&28) != reg_map.get(&29)
                && reg_map.get(&28) != reg_map.get(&30)
        );
        assert!(
            reg_map.get(&29) != reg_map.get(&30)
        );
    }
}
