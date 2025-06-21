use sb_compiler_lirgen_ir::{lir, LirBlock, Bne, JmpLabel};
use sb_compiler_semcheck_hir::For;

use crate::{GenContext, ZERO_REG};
use super::{lirgen_expr, lirgen_block};

pub fn lirgen_for<'input>(ctx: &mut GenContext<'input>, r#for: &For<'input>) -> LirBlock {
    // 初期化節
    let lir_init = lirgen_expr(ctx, &r#for.init);

    // 条件節
    let lir_cond = lirgen_expr(ctx, &r#for.cond);
    let reg_cond = lir_cond.result_reg();

    // 継続節
    let lir_incr = lirgen_expr(ctx, &r#for.incr);

    // ブロック
    let lir_block = lirgen_block(ctx, &r#for.block);

    // ラベル
    let label_cond = ctx.alloc_label();
    let label_end = ctx.alloc_label();

    LirBlock::Multiple {
        lirs: vec![
            lir_init,
            lir!(Label label_cond),
            lir_cond,
            lir!(Bne(12) ZERO_REG, reg_cond, ZERO_REG),
            lir!(JmpLabel(label_end)),
            lir_block,
            lir_incr,
            lir!(JmpLabel(label_cond)),
            lir!(Label label_end),
        ]
    }
}
