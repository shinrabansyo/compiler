use sb_compiler_parse_ast::If;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::{lir, LIR, Pop, Bne, Jmp, Label};

use super::{lirgen_expr, lirgen_block, lirgen_stmt, TMP_REG, ZERO_REG};

pub fn lirgen_if(lirs: &mut Vec<LIR>, r#if: &If, analyze_result: &AnalyzeResult) {
    let else_label = format!("ifelse_stmt.{}.{}", lirs.len(), r#if.namespace);
    let end_label = format!("ifend_stmt.{}.{}", lirs.len(), r#if.namespace);

    // 条件節
    lirgen_expr(lirs, &r#if.cond, analyze_result);
    lirs.push(lir!(Pop TMP_REG));
    lirs.push(lir!(Bne ZERO_REG, TMP_REG, 12));
    lirs.push(lir!(Jmp else_label.clone()));

    // 実行節 (本体)
    lirgen_block(lirs, &r#if.block, analyze_result);
    lirs.push(lir!(Jmp end_label.clone()));

    // 実行節 (else)
    lirs.push(lir!(Label else_label));
    if let Some(else_stmt) = &r#if.else_stmt {
        lirgen_stmt(lirs, else_stmt, analyze_result);
    }
    lirs.push(lir!(Label end_label));
}
