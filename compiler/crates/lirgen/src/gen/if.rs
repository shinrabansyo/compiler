use sb_compiler_lirgen_ir::{lir, LirBlock, Bne, JmpLabel, Nop};
use sb_compiler_semcheck_hir::If;

use super::{GenContext, ZERO_REG, lirgen_expr, lirgen_block, lirgen_stmt};

pub fn lirgen_if(ctx: &mut GenContext, r#if: &If) -> LirBlock {
    // 条件節
    let lir_cond = lirgen_expr(ctx, &r#if.cond);
    let reg_cond = lir_cond.result_reg();

    // True ブロック
    let lir_true_block = lirgen_block(ctx, &r#if.block);

    // False ブロック
    let lir_else_block = if let Some(else_stmt) = &r#if.else_stmt {
        lirgen_stmt(ctx, else_stmt)
    } else {
        lir!(Nop)
    };

    // ラベル
    let label_false = ctx.alloc_label();
    let label_end = ctx.alloc_label();

    LirBlock::Single {
        result_reg: ZERO_REG,
        lirs: vec![
            lir_cond,
            lir!(Bne(12) ZERO_REG, reg_cond, ZERO_REG),
            lir!(JmpLabel(label_false)),
            lir_true_block,
            lir!(JmpLabel(label_end)),
            lir!(Label label_false),
            lir_else_block,
            lir!(Label label_end),
        ],
    }
}
