use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_semcheck_impl_vardecl::{Var, VarDeclChecker};
use sb_compiler_type::r#type::{Bool, Char, NumConst, Type};
use sb_compiler_type::Typed;

use super::{Expr, Call, SemCheck, Dep};

#[derive(Debug)]
pub enum Value<'src> {
    Const {
        span: Span<'src>,
        value: i32,
        value_ty: Arc<Type>,
    },
    Var {
        span: Span<'src>,
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
    async fn check0(ctx: Dep<'_, 'src>, value: ast::Value<'src>) -> miette::Result<Self>
    where
        Self: Sized,
    {
        match value {
            ast::Value::Bool { span, value } => {
                Ok(Value::Const {
                    span,
                    value: if value { 1 } else { 0 },
                    value_ty: Bool.ty(),
                })
            }
            ast::Value::Char { span, value } => {
                Ok(Value::Const {
                    span,
                    value: value as i32,
                    value_ty: Char.ty(),
                })
            }
            ast::Value::Const { span, value } => {
                Ok(Value::Const {
                    span,
                    value,
                    value_ty: NumConst.ty(),
                })
            }
            ast::Value::Var { span, name } => {
                Ok(Value::Var {
                    span,
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

impl<'src> Spanned<'src> for Value<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            Value::Const { span, .. } => *span,
            Value::Var { span, .. } => *span,
            Value::Expr { expr } => expr.span(),
            Value::Call { call } => call.span(),
        }
    }
}

impl Typed for Value<'_> {
    fn ty(&self) -> Arc<Type> {
        match self {
            Value::Const { value_ty, .. } => value_ty.ty(),
            Value::Var { var, .. } => var.ty.ty(),
            Value::Expr { expr, .. } => expr.ty(),
            Value::Call { call } => call.ty(),
        }
    }
}
