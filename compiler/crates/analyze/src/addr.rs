use crate::utils::LayeredTable;
use crate::NodeInfo;

type Table<'a> = LayeredTable<NodeInfo<'a>>;

pub fn analyze_addr(mut table: Table) -> Table {
    // Global 名前空間
    if let Some(global) = table.find_namespace_mut("global") {
        let mut addr = 0;
        for (_, info) in global.iter_mut() {
            info.local_addr = addr;
            addr += 4;
        }
    }

    table
}
