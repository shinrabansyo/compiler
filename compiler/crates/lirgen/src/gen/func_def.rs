use sb_compiler_parse_ast::FuncDef;
use sb_compiler_lirgen_ir::{lir, LirBlock, LirTopElem, FnEpilogue, FnPrologue};

use crate::{GenContext, ZERO_REG};
use super::lirgen_block;

pub fn lirgen_func_def(ctx: &mut GenContext, func: &FuncDef) -> LirTopElem {
    // 本体
    let lir_body = lirgen_block(ctx, &func.block);

    // LirBlock 構成
    let lir_block = LirBlock::Single {
        result_reg: ZERO_REG,
        lirs: vec![
            lir!(FnPrologue),
            lir_body,
            lir!(FnEpilogue),
        ],
    };

    LirTopElem::Function {
        namespace: func.namespace.clone(),
        name: func.ident.clone(),
        body: lir_block,
    }
}
