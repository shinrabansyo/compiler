use sb_compiler_parse::parse;
use sb_compiler_analyze::analyze;
use sb_compiler_lirgen::lirgen;
use sb_compiler_codegen::codegen;

pub fn compile(input: &str) -> anyhow::Result<String> {
    // 1. 構文解析 (&str -> AST)
    let ast = parse(input)?;

    // 2. 意味解析 (AST -> AST + NodeInfo)
    let _ = analyze(&ast)?;

    // 3. LIR生成 (AST -> [LIR])
    let lir = lirgen(&ast);

    // 4. コード生成 ([LIR] -> [Asm])
    let asms = lir
        .into_iter()
        .map(codegen)
        .collect::<Vec<_>>();

    // 5. 文字列へ変換 ([Asm] -> String)
    let mut result = String::new();
    result.push_str("===\n");
    for asm in &asms {
        for inst in &asm.inst {
            result.push_str(&inst.to_string());
            result.push_str("\n");
        }
        result.push_str("\n");
    }

    Ok(result)
}
