use sb_compiler_parse_ast::While;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::{lir, LIR, Pop, Beq, Jmp, Label};

use super::{lirgen_expr, lirgen_stmt, TMP_REG, ZERO_REG};

pub fn lirgen_while(lirs: &mut Vec<LIR>, r#while: &While, analyze_result: &AnalyzeResult) {
    let stmt_label = format!("while_stmt.{}.{}", lirs.len(), r#while.namespace);
    let cond_label = format!("while_cond.{}.{}", lirs.len(), r#while.namespace);

    // 条件節へのジャンプ
    lirs.push(lir!(Jmp cond_label.clone()));

    // 実行部
    lirs.push(lir!(Label stmt_label.clone()));
    lirgen_stmt(lirs, &r#while.stmt, analyze_result);

    // 継続判定
    lirs.push(lir!(Label cond_label));
    lirgen_expr(lirs, &r#while.cond, analyze_result);
    lirs.push(lir!(Pop TMP_REG));
    lirs.push(lir!(Beq ZERO_REG, TMP_REG, 12));
    lirs.push(lir!(Jmp stmt_label));
}
