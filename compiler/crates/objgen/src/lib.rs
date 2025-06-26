use sb_linker::obj::Object;

use sb_compiler_lirgen_ir::LirTopElem;
use sb_compiler_objgen_code::codegen;
use sb_compiler_objgen_opt::optimize;

pub fn objgen<I>(lirs: I) -> Vec<Object>
where
    I: Iterator<Item = LirTopElem>,
{
    let objgen = |lir| {
        let obj = codegen(lir);
        let obj = optimize(obj);
        obj
    };

    lirs.map(objgen)
        .collect::<Vec<_>>()
}
