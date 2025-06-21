use sb_compiler_parse_ast as ast;
use sb_compiler_semcheck_impl_vardecl::{Var, VarDeclChecker};

use super::{Expr, Call, SemCheckFrom, Dep};

#[derive(Debug)]
pub enum Value<'src> {
    Const {
        value: i32,
    },
    Var {
        var: Var<'src>,
    },
    Expr {
        expr: Box<Expr<'src>>,
    },
    Call {
        call: Call<'src>,
    }
}

impl<'src> SemCheckFrom<Dep<'_, 'src>, ast::Value<'src>> for Value<'src> {
    async fn check0(ctx: Dep<'_, 'src>, value: ast::Value<'src>) -> anyhow::Result<Self>
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
