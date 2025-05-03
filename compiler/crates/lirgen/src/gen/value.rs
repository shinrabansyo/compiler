use sb_compiler_parse_ast::Value;
use sb_compiler_lirgen_ir::{lir, LirTree, Add, Call, Li};

use crate::{GenContext, ZERO_REG};
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
            let (mut lirs, mut reg_args) = (vec![], vec![]);
            for arg in &call.args {
                let lir_arg = lirgen_value(ctx, arg);
                let reg_arg = lir_arg.reserved_reg_range().1 - 1;
                lirs.push(lir_arg);
                reg_args.push(reg_arg as u8);
            }
            lirs.push(lir!(Call(format!("{}.{}", call.ident, "global"), reg_args)));
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
