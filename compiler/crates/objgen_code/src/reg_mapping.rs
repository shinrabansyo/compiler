mod coloring;
mod deps_graph;
mod lifetime;

use std::collections::HashMap;

use sb_compiler_lirgen_ir::LirTopElem;

use coloring::coloring;
use deps_graph::build_deps_graph;
use lifetime::analyze_lifetime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MapTo {
    Reg(u8),
    Stack(u32),
}

#[derive(Debug, Default)]
pub struct RegMap {
    spilled_regs: u32,
    map: HashMap<u32, MapTo>,
}

impl From<(u32, HashMap<u32, MapTo>)> for RegMap {
    fn from((spilled_regs, map): (u32, HashMap<u32, MapTo>)) -> Self {
        RegMap {
            spilled_regs,
            map,
        }
    }
}

impl RegMap {
    pub fn spilled_regs(&self) -> i32 {
        self.spilled_regs as i32
    }

    pub fn get(&self, key: &u32) -> MapTo {
        match self.map.get(key) {
            Some(value) => *value,
            None if *key < 20 => MapTo::Reg(*key as u8),
            None => panic!("RegMap does not contain key: {}", key),
        }
    }
}

pub fn mapping(lir: &LirTopElem, usable_regs: &[u8]) -> RegMap {
    // 1. 寿命解析
    let lir_block = match lir {
        LirTopElem::Function { body, .. } => body,
    };
    let lifetime_tracker = analyze_lifetime(lir_block);

    // 2. レジスタ依存グラフの構築
    let deps_graph = build_deps_graph(lifetime_tracker);

    // 3. グラフ彩色問題として解く
    let reg_map = coloring(deps_graph, usable_regs);

    reg_map
}
