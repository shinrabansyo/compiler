use sb_compiler_parse_ast::Value;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::{lir, LIR, Li, Call, Push, Lw};

use super::{lirgen_expr, TMP_REG, ZERO_REG, VARBASE_REG};

pub fn lirgen_value(lirs: &mut Vec<LIR>, value: &Value, analyze_result: &AnalyzeResult) {
    match value {
        Value::Const { value, .. } => {
            lirs.push(lir!(Li TMP_REG, *value));
            lirs.push(lir!(Push TMP_REG));
        }
        Value::Var { namespace, name } => {
            let addr = analyze_result.find(namespace, name).local_addr;
            let base_reg = if namespace == "global" {
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
        Value::Call { ident, .. } => {
            let jmp_to = format!("{}.{}", ident, "global");
            lirs.push(lir!(Call jmp_to));
            lirs.push(lir!(Push TMP_REG));
        }
    }
}
