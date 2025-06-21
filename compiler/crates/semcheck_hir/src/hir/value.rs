use sb_compiler_parse_ast as ast;
use sb_compiler_semcheck_impl_vardecl::{Var, VarDeclChecker};

use super::{Expr, Call, SemCheckFrom, Dep};

#[derive(Debug)]
pub enum Value<'input> {
    Const {
        value: i32,
    },
    Var {
        var: Var<'input>,
    },
    Expr {
        expr: Box<Expr<'input>>,
    },
    Call {
        call: Call<'input>,
    }
}

impl<'input> SemCheckFrom<Dep<'_, 'input>, ast::Value<'input>> for Value<'input> {
    async fn check0(ctx: Dep<'_, 'input>, value: ast::Value<'input>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match value {
            ast::Value::Const { value } => {
                Ok(Value::Const { value })
            }
            ast::Value::Var { name } => {
                Ok(Value::Var {
                    var: VarDeclChecker::find(&mut ctx.var_decl, &name).await?,
                })
            }
            ast::Value::Expr { expr } => {
                Ok(Value::Expr {
                    expr: Box::new(Expr::check(ctx, *expr).await?),
                })
            }
            ast::Value::Call { call } => {
                Ok(Value::Call {
                    call: Call::check(ctx, call).await?,
                })
            }
        }
    }
}
