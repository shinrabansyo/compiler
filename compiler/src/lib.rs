use sb_compiler_parse::parse;
use sb_compiler_analyze::analyze;
use sb_compiler_lirgen::lirgen;
use sb_compiler_codegen::codegen;

pub fn compile(input: &str) -> anyhow::Result<Vec<String>> {
    let ast = parse(input)?;
    let _ = analyze(&ast)?;
    let lirs = lirgen(ast);
    let asms = codegen(lirs);
    Ok(asms)
}
