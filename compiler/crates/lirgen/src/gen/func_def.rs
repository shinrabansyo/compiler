use sb_compiler_parse_ast::FuncDef;
use sb_compiler_lirgen_ir::{lir, LirBlock, LirTopElem, FnEpilogue, FnPrologue};

use crate::{GenContext, ZERO_REG};
use super::lirgen_block;

pub fn lirgen_func_def(ctx: &mut GenContext, func: &FuncDef) -> LirTopElem {
    // 本体
    ctx.set_fn_name(func.namespace.clone(), func.ident.clone());
    let lir_body = lirgen_block(ctx, &func.block);
    ctx.reset_fn_name();

    // LirBlock 構成
    let lir_block = LirBlock::Single {
        result_reg: ZERO_REG,
        lirs: vec![
            lir!(GLabel format!("{}.{}", func.namespace, func.ident)),
            lir!(FnPrologue),
            lir_body,
            lir!(FnEpilogue(format!("{}.{}", func.namespace, func.ident))),
        ],
    };

    LirTopElem::Function {
        namespace: func.namespace.clone(),
        name: func.ident.clone(),
        body: lir_block,
    }
}
