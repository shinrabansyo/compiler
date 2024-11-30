use sb_compiler_parse_ast::For;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::{lir, LIR, Pop, Beq, Jmp, Label};

use super::{lirgen_expr, lirgen_block, TMP_REG, ZERO_REG};

pub fn lirgen_for(lirs: &mut Vec<LIR>, r#for: &For, analyze_result: &AnalyzeResult) {
    let stmt_label = format!("for_stmt.{}.{}", lirs.len(), r#for.namespace);
    let cond_label = format!("for_cond.{}.{}", lirs.len(), r#for.namespace);

    // 初期化節
    lirgen_expr(lirs, &r#for.init, analyze_result);
    lirs.push(lir!(Pop TMP_REG));

    // 判定節へのジャンプ
    lirs.push(lir!(Jmp cond_label.clone()));

    // 実行節
    lirs.push(lir!(Label stmt_label.clone()));
    lirgen_block(lirs, &r#for.block, analyze_result);

    // 更新節
    lirgen_expr(lirs, &r#for.incr, analyze_result);
    lirs.push(lir!(Pop TMP_REG));

    // 判定節
    lirs.push(lir!(Label cond_label));
    lirgen_expr(lirs, &r#for.cond, analyze_result);
    lirs.push(lir!(Pop TMP_REG));
    lirs.push(lir!(Beq ZERO_REG, TMP_REG, 12));
    lirs.push(lir!(Jmp stmt_label));
}
