use sb_compiler_parse_ast::Value;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::{lir, LIR, Li, Add, Call, Push, Pop, Lw};

use super::{lirgen_expr, TMP_REG, ZERO_REG, VARBASE_REG, RET_REG};

pub fn lirgen_value(lirs: &mut Vec<LIR>, value: &Value, analyze_result: &AnalyzeResult) {
    match value {
        Value::Const { value, .. } => {
            lirs.push(lir!(Li TMP_REG, *value));
            lirs.push(lir!(Push TMP_REG));
        }
        Value::Var { namespace, name } => {
            let node_info = analyze_result.find(namespace, name);
            let addr = node_info.local_addr;
            let base_reg = if node_info.namespace == "global" {
                ZERO_REG
            } else {
                VARBASE_REG
            };

            lirs.push(lir!(Lw TMP_REG, base_reg, addr));
            lirs.push(lir!(Push TMP_REG));
        }
        Value::Expr { expr, .. } => {
            lirgen_expr(lirs, expr, analyze_result);
        }
        Value::Call { call, .. } => {
            for (idx, value) in call.args.iter().enumerate() {
                lirgen_value(lirs, value, analyze_result);
                lirs.push(lir!(Pop TMP_REG));

                let reg = (idx + 10) as u8;
                lirs.push(lir!(Li reg, 0));
                lirs.push(lir!(Add reg, TMP_REG));
            }

            let jmp_to = format!("{}.{}", call.ident, "global");
            lirs.push(lir!(Call jmp_to));
            lirs.push(lir!(Push RET_REG));
        }
    }
}
