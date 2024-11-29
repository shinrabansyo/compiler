use sb_compiler_parse_ast::FuncDef;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::{lir, LIR, Label, FSave, FLoad, Return};

use super::lirgen_block;

pub fn lirgen_func_def(lirs: &mut Vec<LIR>, func: &FuncDef, analyze_result: &AnalyzeResult) {
    let label = format!("{}.{}", func.ident, func.namespace);
    lirs.push(lir!(Label label));
    lirs.push(lir!(FSave));

    // TODO: 変数領域確保

    lirgen_block(lirs, &func.block, analyze_result);

    lirs.push(lir!(FLoad));
    lirs.push(lir!(Return));
}
