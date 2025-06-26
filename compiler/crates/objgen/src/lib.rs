use sb_linker::obj::Object;

use sb_compiler_lirgen_ir::LirTopElem;
use sb_compiler_objgen_code::codegen;
use sb_compiler_objgen_opt::optimize;

type LIRs = Vec<LirTopElem>;
type Objects = Vec<Object>;

pub fn objgen(lirs: LIRs) -> Objects {
    let objgen = |lir| {
        let obj = codegen(lir);
        let obj = optimize(obj);
        obj
    };

    lirs.into_iter()
        .map(objgen)
        .collect()
}
