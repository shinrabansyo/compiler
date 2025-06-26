use sb_linker::obj::Object;

use sb_compiler_parse::parse;
use sb_compiler_semcheck::semcheck;
use sb_compiler_lirgen::lirgen;
use sb_compiler_objgen::objgen;

type Texts<'name, 'src> = Vec<(&'name str, &'src str)>;
type Objects = Vec<Object>;

pub fn compile<'name, 'src>(inputs: Texts) -> miette::Result<Objects> {
    // 1. 構文解析 ([&str] -> [AST])
    let asts = parse(inputs)?;

    // 2. 意味解析 ([AST] -> [HIR])
    let hirs = semcheck(asts)?;

    // 3. LIR 生成 ([HIR] -> [LIR])
    let lirs = lirgen(hirs);

    // 4. オブジェクトファイル生成 ([LIR] -> [Obj])
    let objs = objgen(lirs);

    Ok(objs)
}
