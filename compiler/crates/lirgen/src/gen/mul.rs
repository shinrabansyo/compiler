use sb_compiler_lirgen_ir::{lir, LirBlock, Add, And, Blt, Bne, Jmp, JmpLabel, Li, Sub, ShiftL, ShiftR};
use sb_compiler_semcheck_hir::Mul as MulHir;

use super::{GenContext, ZERO_REG, lirgen_cast};

pub fn lirgen_mul<'src>(ctx: &mut GenContext<'src>, add: MulHir<'src>) -> LirBlock {
    let (result_reg, lirs) = match add {
        MulHir::Multiply { lhs, rhs, .. } => {
            let lir_lhs = lirgen_mul(ctx, *lhs);
            let reg_lhs = lir_lhs.result_reg();

            let lir_rhs = lirgen_cast(ctx, rhs);
            let reg_rhs = lir_rhs.result_reg();

            let reg_one = ctx.alloc_reg();
            let reg_pprd = ctx.alloc_reg();
            let reg_cnt = ctx.alloc_reg();
            let reg_result = ctx.alloc_reg();

            let label_cond = ctx.alloc_label();
            let label_end = ctx.alloc_label();

            // # 乗算の疑似コード (a * b)
            // b の下のビットから順番に見ていく
            //   そのビットが 1 なら結果に a を加算
            //   a を左にシフト
            //
            // fn mul(mut a: u32, mut b: u32) -> u32 {
            //   let mut result = 0;
            //   for i in 0..32 {
            //     let t = 0 - (b&1);
            //     result += a & t;
            //     a <<= 1;
            //     b >>= 1;
            //   }
            //
            //   li reg_result, 0
            //   li reg_cnt, 32
            // label_cond:
            //   bne r0, (reg_cnt, r0) -> 12
            //   jmp label_end
            //   andi t0, reg_rhs, 1
            //   sub t0, r0, t0
            //   and t0, reg_lhs, t0
            //   add reg_result, reg_result, t0
            //   slli reg_lhs, reg_lhs, 1
            //   srli reg_rhs, reg_rhs, 1
            //   subi reg_cnt, reg_cnt, 1
            //   jmp mul_cond
            // label_end:
            //   ...

            (
                reg_result,
                vec![
                    lir_lhs,
                    lir_rhs,
                    lir!(Li(0) reg_result),
                    lir!(Li(32) reg_cnt),
                    lir!(Li(1) reg_one),
                    lir!(Label label_cond),
                    lir!(Bne(12) ZERO_REG, reg_cnt, ZERO_REG),
                    lir!(JmpLabel(label_end)),
                    lir!(And reg_pprd, reg_rhs, reg_one),
                    lir!(Sub reg_pprd, ZERO_REG, reg_pprd),
                    lir!(And reg_pprd, reg_lhs, reg_pprd),
                    lir!(Add reg_result, reg_result, reg_pprd),
                    lir!(ShiftL reg_lhs, reg_lhs, reg_one),
                    lir!(ShiftR reg_rhs, reg_rhs, reg_one),
                    lir!(Sub reg_cnt, reg_cnt, reg_one),
                    lir!(JmpLabel(label_cond)),
                    lir!(Label label_end),
                ],
            )
        }
        MulHir::Divide { lhs, rhs, .. } => {
            let lir_lhs = lirgen_mul(ctx, *lhs);
            let reg_lhs = lir_lhs.result_reg();

            let lir_rhs = lirgen_cast(ctx, rhs);
            let reg_rhs = lir_rhs.result_reg();

            let reg_one = ctx.alloc_reg();
            let reg_lhs_copy = ctx.alloc_reg();
            let reg_result = ctx.alloc_reg();

            let label_cond = ctx.alloc_label();
            let label_end = ctx.alloc_label();

            (
                reg_result,
                vec![
                    lir_lhs,
                    lir_rhs,
                    lir!(Add reg_lhs_copy, ZERO_REG, reg_lhs),
                    lir!(Li(1) reg_one),
                    lir!(Li(0) reg_result),
                    lir!(Label label_cond),
                    lir!(Bne(12) ZERO_REG, reg_rhs, ZERO_REG),
                    lir!(JmpLabel(label_end)),
                    lir!(Blt(12) ZERO_REG, reg_lhs_copy, reg_rhs),
                    lir!(Jmp(12)),
                    lir!(JmpLabel(label_end)),
                    lir!(Add reg_result, reg_result, reg_one),
                    lir!(Sub reg_lhs_copy, reg_lhs_copy, reg_rhs),
                    lir!(JmpLabel(label_cond)),
                    lir!(Label label_end),
                ],
            )
        }
        MulHir::Modulo { lhs, rhs, .. } => {
            let lir_lhs = lirgen_mul(ctx, *lhs);
            let reg_lhs = lir_lhs.result_reg();

            let lir_rhs = lirgen_cast(ctx, rhs);
            let reg_rhs = lir_rhs.result_reg();

            let reg_result = ctx.alloc_reg();

            let label_cond = ctx.alloc_label();
            let label_end = ctx.alloc_label();

            (
                reg_result,
                vec![
                    lir_lhs,
                    lir_rhs,
                    lir!(Add reg_result, ZERO_REG, reg_lhs),
                    lir!(Label label_cond),
                    lir!(Bne(18) ZERO_REG, reg_rhs, ZERO_REG),
                    lir!(Add reg_result, ZERO_REG, ZERO_REG),
                    lir!(JmpLabel(label_end)),
                    lir!(Blt(12) ZERO_REG, reg_result, reg_rhs),
                    lir!(Jmp(12)),
                    lir!(JmpLabel(label_end)),
                    lir!(Sub reg_result, reg_result, reg_rhs),
                    lir!(JmpLabel(label_cond)),
                    lir!(Label label_end),
                ],
            )
        }
        MulHir::Cast { value, .. } => {
            return lirgen_cast(ctx, value);
        }
    };

    LirBlock::Single {
        result_reg,
        lirs,
    }
}
