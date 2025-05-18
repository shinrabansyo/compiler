use sb_compiler_parse_ast::FuncDef;
use sb_compiler_lirgen_ir::LirBlock;

use crate::GenContext;
use super::lirgen_block;

pub fn lirgen_func_def(ctx: &mut GenContext, func: &FuncDef) -> LirBlock {
    // TODO
    // let label = format!("{}.{}", func.ident, func.namespace);
    // lirs.push(lir!(Label label));
    // lirs.push(lir!(FSave));
    // lirs.push(lir!(VarAlloc analyze_result.find(&func.namespace, &func.ident).size));

    // for (idx, arg) in func.args.iter().enumerate() {
    //     let addr = analyze_result.find(&arg.namespace, &arg.ident).local_addr;
    //     let reg = (idx + 10) as u8;
    //     lirs.push(lir!(Sw VARBASE_REG, addr, reg));
    // }

    // lirgen_block(lirs, &func.block, analyze_result);

    // lirs.push(lir!(VarFree));
    // lirs.push(lir!(FLoad));
    // lirs.push(lir!(Return));

    lirgen_block(ctx, &func.block)
}
