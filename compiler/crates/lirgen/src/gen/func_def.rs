use sb_compiler_parse_ast::FuncDef;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::{lir, LIR, Sw, Label, FSave, FLoad, VarAlloc, VarFree, Return};

use super::{lirgen_block, VARBASE_REG};

pub fn lirgen_func_def(lirs: &mut Vec<LIR>, func: &FuncDef, analyze_result: &AnalyzeResult) {
    let label = format!("{}.{}", func.ident, func.namespace);
    lirs.push(lir!(Label label));
    lirs.push(lir!(FSave));
    lirs.push(lir!(VarAlloc analyze_result.find(&func.namespace, &func.ident).size));

    for (idx, arg) in func.args.iter().enumerate() {
        let addr = analyze_result.find(&arg.namespace, &arg.ident).local_addr;
        let reg = (idx + 10) as u8;
        lirs.push(lir!(Sw VARBASE_REG, addr, reg));
    }

    lirgen_block(lirs, &func.block, analyze_result);

    lirs.push(lir!(VarFree));
    lirs.push(lir!(FLoad));
    lirs.push(lir!(Return));
}
