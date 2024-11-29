use sb_compiler_parse_ast::{Program, Top, FuncDef};

use crate::utils::LayeredTable;
use crate::NodeInfo;

type Table<'a> = LayeredTable<NodeInfo<'a>>;

pub fn analyze_addr<'ast>(mut table: Table<'ast>, program: &'ast Program) -> Table<'ast> {
    // Global 名前空間
    if let Some(global) = table.find_namespace_mut("global") {
        let mut addr = 0;
        for (_, info) in global.iter_mut() {
            info.local_addr = addr;
            info.size = 4;
            addr += 4;
        }
    }

    // 各関数空間
    for top in &program.top_elems {
        analyze_addr_top(&mut table, top);
    }

    table
}

pub fn analyze_addr_top(table: &mut Table, top: &Top) {
    match top {
        Top::FuncDef { func_def, .. } => {
            analyze_addr_func_def(table, func_def)
        }
        _ => {},
    }
}

pub fn analyze_addr_func_def(table: &mut Table, func_def: &FuncDef) {
    // 関数内の各変数のアドレス
    let func_name = format!("{}.{}", func_def.ident, func_def.namespace);
    let mut addr = 0;
    if let Some(func_namespace) = table.find_namespace_mut(&func_name) {
        for (_, info) in func_namespace.iter_mut() {
            info.local_addr = addr;
            info.size = 4;
            addr += 4;
        }
    }

    // 関数のサイズ
    let func_info = table.find_name_mut(&func_def.namespace, &func_def.ident).unwrap();
    func_info.size = addr;
}
