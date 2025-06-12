use sb_linker::obj::Object;

use sb_compiler_parse::parse;
use sb_compiler_lirgen::lirgen;
use sb_compiler_codegen::codegen;
use sb_compiler_opt::optimize;

pub fn compile(input: &str) -> anyhow::Result<Vec<Object>> {
    // 1. 構文解析 (&str -> AST)
    let ast = parse(input)?;

    // 2. 意味解析 (AST -> AST + NodeInfo)
    // let _ = analyze(&ast)?;

    // 3. LIR生成 (AST -> [LIR])
    let lir = lirgen(&ast);

    // 4. コード生成 & 最適化 ([LIR] -> [Obj] -> [Obj])
    let objs = lir
        .into_iter()
        .map(codegen)
        .map(optimize)
        .collect::<Vec<_>>();

    Ok(objs)
}
