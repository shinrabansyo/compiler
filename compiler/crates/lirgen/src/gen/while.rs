use sb_compiler_parse_ast::While;
use sb_compiler_lirgen_ir::{lir, LirBlock, Bne, JmpLabel};

use crate::{GenContext, ZERO_REG};
use super::{lirgen_expr, lirgen_block};

pub fn lirgen_while(ctx: &mut GenContext, r#while: &While) -> LirBlock {
    // 条件節
    let lir_cond = lirgen_expr(ctx, &r#while.cond);
    let reg_cond = lir_cond.result_reg();

    // ブロック
    let lir_block = lirgen_block(ctx, &r#while.block);

    // ラベル
    let label_cond = ctx.alloc_label();
    let label_end = ctx.alloc_label();

    LirBlock::Multiple {
        lirs: vec![
            lir!(Label label_cond),
            lir_cond,
            lir!(Bne(12) ZERO_REG, reg_cond, ZERO_REG),
            lir!(JmpLabel(label_end)),
            lir_block,
            lir!(JmpLabel(label_cond)),
            lir!(Label label_end),
        ]
    }
}
