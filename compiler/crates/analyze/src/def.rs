use sb_compiler_parse_ast::{AST, TopLevel, ConstDecl};

use crate::utils::LayeredTable;
use crate::NodeInfo;

type Table<'a> = LayeredTable<NodeInfo<'a>>;

pub fn analyze_defs(ast: &AST) -> anyhow::Result<Table> {
    let mut table = LayeredTable::new();
    for top_level in &ast.top_level_elems {
        analyze_def_top_level(&mut table, top_level)?;
    }
    Ok(table)
}

fn analyze_def_top_level<'ast>(
    table: &mut Table<'ast>,
    ast: &'ast TopLevel,
) -> anyhow::Result<()> {
    match ast {
        TopLevel::ConstDecl { const_decl, .. } => {
            analyze_defs_const_decl(table, const_decl)?;
        }
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
