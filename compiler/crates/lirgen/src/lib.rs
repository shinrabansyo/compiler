use sb_compiler_parse_ast::{AST, Expr, Value};
use sb_compiler_lirgen_ir::{LIR, Li, Add, Sub, Push, Pop};

const TMP_REG: u8   = 4;  // r4
const TMP_REG_L: u8 = 4;  // r4
const TMP_REG_R: u8 = 5;  // r5

pub fn lirgen(ast: AST) -> Vec<LIR> {
    match ast {
        AST::Expr(expr) => {
            let mut insts = Vec::new();
            lirgen_expr(&mut insts, *expr);
            insts
        }
    }
}

fn lirgen_expr(insts: &mut Vec<LIR>, expr: Expr) {
    match expr {
        Expr::Plus(lhs, rhs) => {
            lirgen_expr(insts, *lhs);
            lirgen_value(insts, rhs);
            insts.push(LIR::Pop(Pop::new(TMP_REG_R)));
            insts.push(LIR::Pop(Pop::new(TMP_REG_L)));
            insts.push(LIR::Add(Add::new(TMP_REG_L, TMP_REG_R)));
            insts.push(LIR::Push(Push::new(TMP_REG_L)));
        }
        Expr::Minus(lhs, rhs) => {
            lirgen_expr(insts, *lhs);
            lirgen_value(insts, rhs);
            insts.push(LIR::Pop(Pop::new(TMP_REG_R)));
            insts.push(LIR::Pop(Pop::new(TMP_REG_L)));
            insts.push(LIR::Sub(Sub::new(TMP_REG_L, TMP_REG_R)));
            insts.push(LIR::Push(Push::new(TMP_REG_L)));
        }
        Expr::Value(value) => {
            lirgen_value(insts, value);
        }
    }
}

fn lirgen_value(insts: &mut Vec<LIR>, value: Value) {
    match value {
        Value::Expr(expr) => {
            lirgen_expr(insts, *expr);
        }
        Value::Const(value) => {
            insts.push(LIR::Li(Li::new(TMP_REG, value)));
            insts.push(LIR::Push(Push::new(TMP_REG)));
        }
    }
}
