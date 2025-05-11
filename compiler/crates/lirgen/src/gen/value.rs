use sb_compiler_parse_ast::Value;
use sb_compiler_lirgen_ir::{lir, LirTree, Add, Call, Li};

use crate::{GenContext, ZERO_REG, FARG_REG_BASE};
use super::lirgen_expr;

pub fn lirgen_value(ctx: &mut GenContext, value: &Value) -> LirTree {
    let reserved_reg_range_start = ctx.reserved_regs;
    let reserved_label_range_start = ctx.reserved_labels;

    let lirs = match value {
        Value::Const { value, .. } => {
            vec![lir!(Li(*value) ctx.alloc_reg())]
        }
        Value::Var { name, .. } => {
            vec![lir!(Add ctx.alloc_reg(), ZERO_REG, *ctx.sym_table.get(name).unwrap())]
        }
        Value::Expr { expr, .. } => {
            let lir_expr = lirgen_expr(ctx, expr);
            let reg_result = lir_expr.reserved_reg_range().1 - 1;

            vec![lir_expr, lir!(Add ctx.alloc_reg(), ZERO_REG, reg_result)]
        }
        Value::Call { call, .. } => {
            let mut lirs = vec![];
            for (idx, value) in call.args.iter().enumerate() {
                let lir_arg = lirgen_value(ctx, value);
                let reg_arg = lir_arg.reserved_reg_range().1 - 1;
                lirs.push(lir_arg);
                lirs.push(lir!(Add FARG_REG_BASE + idx as u32, ZERO_REG, reg_arg));
            }
            lirs.push(lir!(Call(format!("{}.{}", call.ident, "global"))));
            lirs
        }
    };

    let reserved_reg_range = (reserved_reg_range_start, ctx.reserved_regs);
    let reserved_label_range = (reserved_label_range_start, ctx.reserved_labels);

    LirTree::Node {
        reserved_reg_range,
        reserved_label_range,
        lirs,
    }
}
