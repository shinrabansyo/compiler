use sb_compiler_parse::parse;
use sb_compiler_lirgen::lirgen;
use sb_compiler_codegen::codegen;

pub fn compile(input: &str) -> Vec<String> {
    let ast = parse(input).unwrap();
    let lir = lirgen(ast);
    codegen(lir)
}
