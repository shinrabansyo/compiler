use sb_compiler_parse::parse;
use sb_compiler_analyze::analyze;
use sb_compiler_lirgen::lirgen;
use sb_compiler_codegen::codegen;

pub fn compile(input: &str) -> anyhow::Result<String> {
    // 1. 構文解析 (&str -> AST)
    let ast = parse(input)?;

    // 2. 意味解析 (AST -> AST + NodeInfo)
    let analyze_result = analyze(&ast)?;

    // 3. LIR生成 (AST + NodeInfo -> LIR)
    let lirs = lirgen(&ast, analyze_result);

    // 4. コード生成 (LIR -> String)
    let asm = codegen(lirs);

    Ok(asm)
}
