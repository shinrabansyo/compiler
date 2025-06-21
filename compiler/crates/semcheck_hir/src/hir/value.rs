use sb_compiler_parse_ast as ast;
use sb_compiler_semcheck_impl_vardecl::{VarDeclChecker, VarId};

use super::{Expr, Call, SemCheckFrom, Dep};

#[derive(Debug)]
pub enum Value<'input> {
    Const {
        value: i32,
    },
    Var {
        id: VarId,
    },
    Expr {
        expr: Box<Expr<'input>>,
    },
    Call {
        call: Call<'input>,
    }
}

impl<'input> SemCheckFrom<Dep<'_>, ast::Value<'input>> for Value<'input> {
    async fn check0(ctx: Dep<'_>, value: ast::Value<'input>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match value {
            ast::Value::Const { value } => {
                Ok(Value::Const { value })
            }
            ast::Value::Var { name } => {
                let var_id = VarDeclChecker::find(
                    &ctx.var_decl,
                    name.as_str(),
                ).await?;

                Ok(Value::Var { id: var_id })
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
