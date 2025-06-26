use sb_linker::obj::Object;

use sb_compiler_parse::parse;
use sb_compiler_semcheck::semcheck;
use sb_compiler_lirgen::lirgen;
use sb_compiler_codegen::codegen;
use sb_compiler_opt::optimize;

pub fn compile<'name, 'src, I>(inputs: I) -> miette::Result<Vec<Object>>
where
    I: Iterator<Item = (&'name str, &'src str)>,
{
    // 1. 構文解析 ([&str] -> [AST])
    let asts = parse(inputs)?;

    // 2. 意味解析 ([AST] -> [HIR])
    let hirs = semcheck(asts.into_iter())?;

    // 3. LIR生成 ([HIR] -> [LIR])
    let lirs = lirgen(hirs.into_iter());

    // 4. コード生成 & 最適化 ([LIR] -> [Obj] -> [Obj])
    let objs = lirs
        .into_iter()
        .map(codegen)
        .map(optimize)
        .collect::<Vec<_>>();

    Ok(objs)
}
