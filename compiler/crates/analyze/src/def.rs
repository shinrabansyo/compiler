use sb_compiler_parse_ast::{Program, Top, ConstDecl};

use crate::utils::LayeredTable;
use crate::NodeInfo;

type Table<'a> = LayeredTable<NodeInfo<'a>>;

pub fn analyze_defs(program: &Program) -> anyhow::Result<Table> {
    let mut table = LayeredTable::new();
    for top in &program.top_elems {
        analyze_def_top(&mut table, top)?;
    }
    Ok(table)
}

fn analyze_def_top<'ast>(
    table: &mut Table<'ast>,
    top: &'ast Top,
) -> anyhow::Result<()> {
    match top {
        Top::ConstDecl { const_decl, .. } => {
            analyze_defs_const_decl(table, const_decl)?;
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn analyze_defs_const_decl<'ast>(
    table: &mut Table<'ast>,
    const_decl: &'ast ConstDecl,
) -> anyhow::Result<()> {
    let info = NodeInfo {
        ty: &const_decl.ty,
        local_addr: 0,
    };
    table.put(&const_decl.namespace, &const_decl.ident, info)
}
