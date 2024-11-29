use sb_compiler_parse_ast::{Program, Top, FuncDef, Stmt, ConstDecl};

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
        Top::FuncDef { func_def, .. } => {
            analyze_defs_func_def(table, func_def)?;
        }
        Top::ConstDecl { const_decl, .. } => {
            analyze_defs_const_decl(table, const_decl)?;
        }
    }
    Ok(())
}

fn analyze_defs_func_def<'ast>(
    table: &mut Table<'ast>,
    func_def: &'ast FuncDef,
) -> anyhow::Result<()> {
    // 関数シグネチャ
    let info = NodeInfo {
        ty: &func_def.ident,
        local_addr: 0,
        size: 0,
    };
    table.put(&func_def.namespace, &func_def.ident, info)?;

    // 引数
    for arg in &func_def.args {
        let info = NodeInfo {
            ty: &arg.ty,
            local_addr: 0,
            size: 0,
        };
        table.put(&arg.namespace, &arg.ident, info)?;
    }

    // 文
    for stmt in &func_def.block.stmts {
        analyze_defs_stmt(table, stmt)?;
    }

    Ok(())
}

fn analyze_defs_stmt<'ast>(
    table: &mut Table<'ast>,
    stmt: &'ast Stmt,
) -> anyhow::Result<()> {
    match stmt {
        Stmt::ConstDecl { const_decl, .. } => {
            analyze_defs_const_decl(table, const_decl)
        }
        Stmt::Block { block, .. } => {
            for stmt in &block.stmts {
                analyze_defs_stmt(table, stmt)?;
            }
            Ok(())
        }
        _ => Ok(())
    }
}

fn analyze_defs_const_decl<'ast>(
    table: &mut Table<'ast>,
    const_decl: &'ast ConstDecl,
) -> anyhow::Result<()> {
    // 定数名
    let info = NodeInfo {
        ty: &const_decl.ty,
        local_addr: 0,
        size: 0,
    };
    table.put(&const_decl.namespace, &const_decl.ident, info)
}
