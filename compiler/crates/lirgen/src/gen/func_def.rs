use sb_compiler_lirgen_ir::{lir, Add, LirBlock, LirTopElem, FnEpilogue, FnPrologue};
use sb_compiler_semcheck_hir::FuncDef;

use super::{GenContext, ZERO_REG, lirgen_block};

pub fn lirgen_func_def(ctx: &mut GenContext, func: &FuncDef) -> LirTopElem {
    // 引数の初期化
    let mut lirs = vec![];
    for (idx, arg) in func.args.iter().enumerate() {
        let reg_arg = ctx.alloc_reg();
        ctx.set_var_reg(arg.var.symbol, reg_arg);
        lirs.push(lir!(Add reg_arg, ZERO_REG, (10 + idx) as u32));
    }
    let lir_init_args_block = LirBlock::Single {
        result_reg: ZERO_REG,
        lirs,
    };

    // 本体
    let lir_body = lirgen_block(ctx, &func.block);

    // LirBlock 構成
    let lir_block = LirBlock::Single {
        result_reg: ZERO_REG,
        lirs: vec![
            lir!(FnPrologue),
            lir_init_args_block,
            lir_body,
            lir!(FnEpilogue),
        ],
    };

    LirTopElem::Function {
        name: func.ident.as_str().to_string(),
        body: lir_block,
    }
}
