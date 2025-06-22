use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_semcheck_impl_vardecl::{Var, VarDeclChecker};
use sb_compiler_type::r#type::{NumConst, Primitive, Type};
use sb_compiler_type::Typed;

use super::{Expr, Call, SemCheck, Dep};

#[derive(Debug)]
pub enum Value<'src> {
    Const {
        value: i32,
        value_ty: Arc<Type>,
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

impl<'src> SemCheck<Dep<'_, 'src>, ast::Value<'src>> for Value<'src> {
    async fn check0(ctx: Dep<'_, 'src>, value: ast::Value<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match value {
            ast::Value::Const { value } => {
                Ok(Value::Const {
                    value,
                    value_ty: Arc::new(Primitive(NumConst)),
                })
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

impl Typed for Value<'_> {
    fn ty(&self) -> &Arc<Type> {
        match self {
            Value::Const { value_ty, .. } => value_ty,
            Value::Var { var, .. } => &var.ty,
            Value::Expr { expr, .. } => expr.ty(),
            Value::Call { call } => call.ty(),
        }
    }
}
