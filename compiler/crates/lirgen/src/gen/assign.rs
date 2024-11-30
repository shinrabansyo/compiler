use sb_compiler_parse_ast::Assign;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::{lir,LIR, Push, Pop, Sw};

use super::{lirgen_logic_or, TMP_REG, VARBASE_REG, ZERO_REG};

pub fn lirgen_assign(lirs: &mut Vec<LIR>, assign: &Assign, analyze_result: &AnalyzeResult) {
    match assign {
        Assign::Assign { namespace, ident, assign: child_assign, .. } => {
            lirgen_assign(lirs, child_assign, analyze_result);

            let node_info = analyze_result.find(namespace, ident);
            let addr = node_info.local_addr;
            let base_reg = if node_info.namespace == "global" {
                ZERO_REG
            } else {
                VARBASE_REG
            };

            lirs.push(lir!(Pop TMP_REG));
            lirs.push(lir!(Sw base_reg, addr, TMP_REG));
            lirs.push(lir!(Push TMP_REG));
        }
        Assign::LogicOr { or, .. } => {
            lirgen_logic_or(lirs, or, analyze_result);
        }
    }
}
