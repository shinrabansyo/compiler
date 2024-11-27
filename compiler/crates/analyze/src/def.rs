use sb_compiler_parse_ast::{AST, ConstDecl};

use crate::utils::LayeredTable;
use crate::NodeInfo;

type Table<'a> = LayeredTable<NodeInfo<'a>>;

pub fn analyze_defs(ast: &AST) -> anyhow::Result<Table> {
    let mut table = LayeredTable::new();
    match ast {
        AST::ConstDecl { const_decl, .. } => {
            analyze_defs_const_decl(&mut table, const_decl)?;
        }
    }
    Ok(table)
}

fn analyze_defs_const_decl<'ast>(
    table: &mut LayeredTable<NodeInfo<'ast>>,
    const_decl: &'ast ConstDecl
) -> anyhow::Result<()> {
    let info = NodeInfo {
        ty: &const_decl.ty,
        local_addr: 0,
    };
    table.put(&const_decl.namespace, &const_decl.ident, info)
}
