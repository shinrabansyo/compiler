use sb_compiler_parse_ast::{AST, ConstDecl, Expr, Value};
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::{LIR, Li, Add, Sub, Push, Pop, Sw};

const ZERO_REG: u8  = 0;  // r0
const TMP_REG: u8   = 4;  // r4
const TMP_REG_L: u8 = 4;  // r4
const TMP_REG_R: u8 = 5;  // r5

pub fn lirgen(ast: &AST, analyze_result: AnalyzeResult) -> Vec<LIR> {
    let mut lirgen_state = LirGenState::from(analyze_result);
    lirgen_state.lirgen_ast(ast);
    lirgen_state.lirs
}

struct LirGenState<'ast> {
    lirs: Vec<LIR>,
    analyze_result: AnalyzeResult<'ast>,
}

impl<'ast> From<AnalyzeResult<'ast>> for LirGenState<'ast> {
    fn from(analyze_result: AnalyzeResult<'ast>) -> Self {
        LirGenState {
            lirs: Vec::new(),
            analyze_result,
        }
    }
}

impl<'ast> LirGenState<'ast> {
    fn lirgen_ast(&mut self, ast: &AST) {
        match ast {
            AST::ConstDecl { const_decl, .. } => {
                self.lirgen_const_decl(const_decl);
            }
        }
    }

    fn lirgen_const_decl(&mut self, const_decl: &ConstDecl) {
        self.lirgen_expr(&const_decl.expr);

        let addr = self
            .analyze_result
            .define_table
            .find(&const_decl.namespace, &const_decl.ident)
            .unwrap()
            .local_id;
        let base_reg = if const_decl.namespace == "global" {
            ZERO_REG
        } else {
            unimplemented!()
        };

        self.lirs.push(LIR::Pop(Pop::new(TMP_REG)));
        self.lirs.push(LIR::Sw(Sw::new(addr, base_reg, TMP_REG)));
    }

    fn lirgen_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Plus { lhs, rhs, .. }=> {
                self.lirgen_expr(lhs);
                self.lirgen_value(rhs);
                self.lirs.push(LIR::Pop(Pop::new(TMP_REG_R)));
                self.lirs.push(LIR::Pop(Pop::new(TMP_REG_L)));
                self.lirs.push(LIR::Add(Add::new(TMP_REG_L, TMP_REG_R)));
                self.lirs.push(LIR::Push(Push::new(TMP_REG_L)));
            }
            Expr::Minus { lhs, rhs, .. } => {
                self.lirgen_expr(lhs);
                self.lirgen_value(rhs);
                self.lirs.push(LIR::Pop(Pop::new(TMP_REG_R)));
                self.lirs.push(LIR::Pop(Pop::new(TMP_REG_L)));
                self.lirs.push(LIR::Sub(Sub::new(TMP_REG_L, TMP_REG_R)));
                self.lirs.push(LIR::Push(Push::new(TMP_REG_L)));
            }
            Expr::Value { value, .. } => {
                self.lirgen_value(value);
            }
        }
    }

    fn lirgen_value(&mut self, value: &Value) {
        match value {
            Value::Expr { expr, .. } => {
                self.lirgen_expr(expr);
            }
            Value::Const { value, .. } => {
                self.lirs.push(LIR::Li(Li::new(TMP_REG, *value)));
                self.lirs.push(LIR::Push(Push::new(TMP_REG)));
            }
        }
    }
}
