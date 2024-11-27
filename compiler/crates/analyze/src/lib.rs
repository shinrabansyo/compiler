mod addr;
mod def;
mod utils;

use sb_compiler_parse_ast::Program;

use addr::analyze_addr;
use def::analyze_defs;
use utils::LayeredTable;

pub struct NodeInfo<'ast> {
    pub ty: &'ast str,
    pub local_addr: u32,
}

pub struct AnalyzeResult<'ast> {
    table: LayeredTable<NodeInfo<'ast>>,
}

impl<'ast> AnalyzeResult<'ast> {
    pub fn find(&self, namespace: &str, name: &str) -> &NodeInfo<'ast> {
        self.table.find_name(namespace, name).unwrap()
    }
}

pub fn analyze(program: &Program) -> anyhow::Result<AnalyzeResult> {
    let table = analyze_defs(program)?;
    let table = analyze_addr(table);
    Ok(AnalyzeResult { table })
}
