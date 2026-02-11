use sb_compiler_lirgen_ir::{lir, LirBlock, Add, And, Beq, Ble, Bne, JmpLabel, Li, Or, Sub, ShiftL, ShiftR, Xor};
use sb_compiler_semcheck_hir::Mul as MulHir;

use super::{GenContext, ZERO_REG, lirgen_cast};

pub fn lirgen_mul<'src>(ctx: &mut GenContext<'src>, add: MulHir<'src>) -> LirBlock {
    let (result_reg, lirs) = match add {
        MulHir::Multiply { lhs, rhs, .. } => {
            let lir_lhs = lirgen_mul(ctx, *lhs);
            let reg_lhs = lir_lhs.result_reg();

            let lir_rhs = lirgen_cast(ctx, rhs);
            let reg_rhs = lir_rhs.result_reg();

            let reg_lhs_copy = ctx.alloc_reg();
            let reg_rhs_copy = ctx.alloc_reg();
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
                    lir!(Add reg_lhs_copy, ZERO_REG, reg_lhs),
                    lir!(Add reg_rhs_copy, ZERO_REG, reg_rhs),
                    lir!(Li(0) reg_result),
                    lir!(Li(32) reg_cnt),
                    lir!(Li(1) reg_one),
                    lir!(Label label_cond),
                    lir!(Bne(12) ZERO_REG, reg_cnt, ZERO_REG),
                    lir!(JmpLabel(label_end)),
                    lir!(And reg_pprd, reg_rhs_copy, reg_one),
                    lir!(Sub reg_pprd, ZERO_REG, reg_pprd),
                    lir!(And reg_pprd, reg_lhs_copy, reg_pprd),
                    lir!(Add reg_result, reg_result, reg_pprd),
                    lir!(ShiftL reg_lhs_copy, reg_lhs_copy, reg_one),
                    lir!(ShiftR reg_rhs_copy, reg_rhs_copy, reg_one),
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

            // # 除算の疑似コード (a / b)
            //        0011
            //      ----------
            // 0011 | 1010 (=10)
            //         110 (= 6)
            //        ----
            //         100
            //          11
            //        ----
            //           1
            //
            //   li reg_one, 1
            //   li reg_result, 0
            //   li reg_cnt, 31
            // label_cond:
            //   srl reg_adivi, reg_lhs, reg_cnt
            //   ble r0, (reg_rhs, reg_adivi) -> 12
            //   jmp label_decr
            //   sll reg_bmuli, reg_rhs, reg_cnt
            //   sub reg_lhs, reg_lhs, reg_bmuli
            //   sll reg_1muli, reg_one, reg_cnt
            //   or reg_result, reg_result, reg_1muli
            // label_decr:
            //   beq r0, (reg_cnt, r0) -> 18
            //   sub reg_cnt, reg_cnt, reg_one
            //   jmp label_cond
            // label_end:
            //   ...

            let reg_lhs_copy = ctx.alloc_reg();
            let reg_rhs_copy = ctx.alloc_reg();
            let reg_sign = ctx.alloc_reg();
            let reg_thirtyone = ctx.alloc_reg();

            let lir_sign_check = LirBlock::Single {
                result_reg: 0,
                lirs: vec![
                    lir!(Add reg_lhs_copy, ZERO_REG, reg_lhs),
                    lir!(Add reg_rhs_copy, ZERO_REG, reg_rhs),
                    lir!(Xor reg_sign, reg_lhs_copy, reg_rhs_copy),
                    lir!(Li(31) reg_thirtyone),
                    lir!(ShiftR reg_sign, reg_sign, reg_thirtyone),
                    lir!(Ble(12) ZERO_REG, ZERO_REG, reg_lhs_copy),
                    lir!(Sub reg_lhs_copy, ZERO_REG, reg_lhs_copy),
                    lir!(Ble(12) ZERO_REG, ZERO_REG, reg_rhs_copy),
                    lir!(Sub reg_rhs_copy, ZERO_REG, reg_rhs_copy),
                ],
            };

            let reg_one = ctx.alloc_reg();
            let reg_cnt = ctx.alloc_reg();
            let reg_adivi = ctx.alloc_reg();
            let reg_bmuli = ctx.alloc_reg();
            let reg_1muli = ctx.alloc_reg();
            let reg_result = ctx.alloc_reg();

            let label_cond = ctx.alloc_label();
            let label_decr = ctx.alloc_label();
            let label_end = ctx.alloc_label();

            let lir_div = LirBlock::Multiple {
                lirs: vec![
                    lir!(Li(1) reg_one),
                    lir!(Li(0) reg_result),
                    lir!(Li(31) reg_cnt),
                    lir!(Label label_cond),
                    lir!(ShiftR reg_adivi, reg_lhs_copy, reg_cnt),
                    lir!(Ble(12) ZERO_REG, reg_rhs_copy, reg_adivi),
                    lir!(JmpLabel(label_decr)),
                    lir!(ShiftL reg_bmuli, reg_rhs_copy, reg_cnt),
                    lir!(Sub reg_lhs_copy, reg_lhs_copy, reg_bmuli),
                    lir!(ShiftL reg_1muli, reg_one, reg_cnt),
                    lir!(Or reg_result, reg_result, reg_1muli),
                    lir!(Label label_decr),
                    lir!(Beq(18) ZERO_REG, reg_cnt, ZERO_REG),
                    lir!(Sub reg_cnt, reg_cnt, reg_one),
                    lir!(JmpLabel(label_cond)),
                    lir!(Label label_end)
                ],
            };

            let lir_sign_apply = LirBlock::Single {
                result_reg: 0,
                lirs: vec![
                    lir!(Beq(12) ZERO_REG, ZERO_REG, reg_sign),
                    lir!(Sub reg_result, ZERO_REG, reg_result),
                ],
            };

            (reg_result, vec![lir_lhs, lir_rhs, lir_sign_check, lir_div, lir_sign_apply])
        }
        MulHir::Modulo { lhs, rhs, .. } => {
            let lir_lhs = lirgen_mul(ctx, *lhs);
            let reg_lhs = lir_lhs.result_reg();

            let lir_rhs = lirgen_cast(ctx, rhs);
            let reg_rhs = lir_rhs.result_reg();

            // # 除算の疑似コード (a / b)
            //        0011
            //      ----------
            // 0011 | 1010 (=10)
            //         110 (= 6)
            //        ----
            //         100
            //          11
            //        ----
            //           1
            //
            //   li reg_one, 1
            //   li reg_result, 0
            //   li reg_cnt, 31
            // label_cond:
            //   srl reg_adivi, reg_lhs, reg_cnt
            //   ble r0, (reg_rhs, reg_adivi) -> 12
            //   jmp label_decr
            //   sll reg_bmuli, reg_rhs, reg_cnt
            //   sub reg_lhs, reg_lhs, reg_bmuli
            //   sll reg_1muli, reg_one, reg_cnt
            //   or reg_result, reg_result, reg_1muli
            // label_decr:
            //   beq r0, (reg_cnt, r0) -> 18
            //   sub reg_cnt, reg_cnt, reg_one
            //   jmp label_cond
            // label_end:
            //   ...

            let reg_lhs_copy = ctx.alloc_reg();
            let reg_rhs_copy = ctx.alloc_reg();
            let reg_sign = ctx.alloc_reg();
            let reg_thirtyone = ctx.alloc_reg();

            let lir_sign_check = LirBlock::Single {
                result_reg: 0,
                lirs: vec![
                    lir!(Add reg_lhs_copy, ZERO_REG, reg_lhs),
                    lir!(Add reg_rhs_copy, ZERO_REG, reg_rhs),
                    lir!(Li(31) reg_thirtyone),
                    lir!(ShiftR reg_sign, reg_lhs_copy, reg_thirtyone),
                    lir!(Ble(12) ZERO_REG, ZERO_REG, reg_lhs_copy),
                    lir!(Sub reg_lhs_copy, ZERO_REG, reg_lhs_copy),
                    lir!(Ble(12) ZERO_REG, ZERO_REG, reg_rhs_copy),
                    lir!(Sub reg_rhs_copy, ZERO_REG, reg_rhs_copy),
                ],
            };

            let reg_one = ctx.alloc_reg();
            let reg_cnt = ctx.alloc_reg();
            let reg_adivi = ctx.alloc_reg();
            let reg_bmuli = ctx.alloc_reg();
            let reg_1muli = ctx.alloc_reg();
            let reg_result = ctx.alloc_reg();

            let label_cond = ctx.alloc_label();
            let label_decr = ctx.alloc_label();
            let label_end = ctx.alloc_label();

            let lir_div = LirBlock::Multiple {
                lirs: vec![
                    lir!(Li(1) reg_one),
                    lir!(Li(0) reg_result),
                    lir!(Li(31) reg_cnt),
                    lir!(Label label_cond),
                    lir!(ShiftR reg_adivi, reg_lhs_copy, reg_cnt),
                    lir!(Ble(12) ZERO_REG, reg_rhs_copy, reg_adivi),
                    lir!(JmpLabel(label_decr)),
                    lir!(ShiftL reg_bmuli, reg_rhs_copy, reg_cnt),
                    lir!(Sub reg_lhs_copy, reg_lhs_copy, reg_bmuli),
                    lir!(ShiftL reg_1muli, reg_one, reg_cnt),
                    lir!(Or reg_result, reg_result, reg_1muli),
                    lir!(Label label_decr),
                    lir!(Beq(18) ZERO_REG, reg_cnt, ZERO_REG),
                    lir!(Sub reg_cnt, reg_cnt, reg_one),
                    lir!(JmpLabel(label_cond)),
                    lir!(Label label_end)
                ],
            };

            let lir_sign_apply = LirBlock::Single {
                result_reg: 0,
                lirs: vec![
                    lir!(Beq(12) ZERO_REG, ZERO_REG, reg_sign),
                    lir!(Sub reg_lhs_copy, ZERO_REG, reg_lhs_copy),
                ],
            };

            (reg_lhs_copy, vec![lir_lhs, lir_rhs, lir_sign_check, lir_div, lir_sign_apply])
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
