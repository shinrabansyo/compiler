use sb_compiler_parse_ast::FuncDef;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::{lir, LIR, Label, FSave, FLoad, VarAlloc, VarFree, Return};

use super::lirgen_block;

pub fn lirgen_func_def(lirs: &mut Vec<LIR>, func: &FuncDef, analyze_result: &AnalyzeResult) {
    let label = format!("{}.{}", func.ident, func.namespace);
    lirs.push(lir!(Label label));
    lirs.push(lir!(FSave));

    let var_size = analyze_result.find(&func.namespace, &func.ident).size;
    lirs.push(lir!(VarAlloc var_size));
    lirgen_block(lirs, &func.block, analyze_result);
    lirs.push(lir!(VarFree var_size));

    lirs.push(lir!(FLoad));
    lirs.push(lir!(Return));
}
