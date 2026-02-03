use sb_compiler_lirgen_ir::{lir, LirBlock, Add, Call as CallInst};
use sb_compiler_semcheck_hir::Call;

use super::{GenContext, FARG_REG_BASE, RET_REG, ZERO_REG, lirgen_expr};

pub fn lirgen_call<'src>(ctx: &mut GenContext<'src>, call: Call<'src>) -> LirBlock {
    let mut lirs = vec![];
    for (idx, value) in call.args.into_iter().enumerate() {
        let lir_arg = lirgen_expr(ctx, value);
        let reg_arg = lir_arg.result_reg();
        lirs.push(lir_arg);
        lirs.push(lir!(Add FARG_REG_BASE + idx as u32, ZERO_REG, reg_arg));
    }
    lirs.push(lir!(CallInst(call.name)));

    LirBlock::Single {
        result_reg: RET_REG,
        lirs,
    }
}
