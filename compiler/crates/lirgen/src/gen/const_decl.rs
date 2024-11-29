use sb_compiler_parse_ast::ConstDecl;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::{lir, LIR, Pop, Sw};

use super::{lirgen_expr, TMP_REG, VARBASE_REG, ZERO_REG};

pub fn lirgen_const_decl(lirs: &mut Vec<LIR>, const_decl: &ConstDecl, analyze_result: &AnalyzeResult) {
    lirgen_expr(lirs, &const_decl.expr, analyze_result);

    let node_info = analyze_result.find(&const_decl.namespace, &const_decl.ident);
    let addr = node_info.local_addr;
    let base_reg = if node_info.namespace == "global" {
        ZERO_REG
    } else {
        VARBASE_REG
    };

    lirs.push(lir!(Pop TMP_REG));
    lirs.push(lir!(Sw base_reg, addr, TMP_REG));
}
