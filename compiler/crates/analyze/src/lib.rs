use sb_compiler_parse_ast::AST;
use sb_compiler_analyze_def::DefineTable;

pub struct AnalyzeResult<'ast> {
    pub define_table: DefineTable<'ast>,
}

pub fn analyze(ast: &AST) -> anyhow::Result<AnalyzeResult> {
    Ok(AnalyzeResult {
        define_table: DefineTable::from(ast)?,
    })
}
