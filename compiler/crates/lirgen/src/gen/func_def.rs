use sb_compiler_parse_ast::FuncDef;
use sb_compiler_lirgen_ir::{lir, LirBlock, LirTopElem, FnEpilogue, FnPrologue};

use crate::{GenContext, ZERO_REG};
use super::lirgen_block;

pub fn lirgen_func_def<'input>(ctx: &mut GenContext<'input>, func: &FuncDef<'input>) -> LirTopElem {
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
        name: func.ident.to_string(),
        body: lir_block,
    }
}
