use sb_linker::obj::Object;

use sb_compiler_parse::parse;
use sb_compiler_semcheck::semcheck;
use sb_compiler_lirgen::lirgen;
use sb_compiler_codegen::codegen;
use sb_compiler_opt::optimize;

pub fn compile<'name, 'src, I>(mut inputs: I) -> miette::Result<Vec<Object>>
where
    I: Iterator<Item = (&'name str, &'src str)>,
{
    // 1. 構文解析 (&str -> AST)
    let ast = parse(inputs.next().unwrap().1)?;

    // 2. 意味解析 (AST -> HIR)
    let hir = semcheck(ast)?;

    // 3. LIR生成 (HIR -> [LIR])
    let lir = lirgen(&hir);

    // 4. コード生成 & 最適化 ([LIR] -> [Obj] -> [Obj])
    let objs = lir
        .into_iter()
        .map(codegen)
        .map(optimize)
        .collect::<Vec<_>>();

    Ok(objs)
}
