use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_semcheck_impl_var::{Var, var_find};
use sb_compiler_semcheck_impl_typedecl::TypeDeclChecker;
use sb_compiler_type::op::ty_can_call;
use sb_compiler_type::r#type::{Bool, Char, NumConst, Type};
use sb_compiler_type::Typed;

use super::{Expr, SemCheck, Dep};

#[derive(Debug)]
pub enum Value<'src> {
    Const {
        span: Span<'src>,
        value: i32,
        ty: Arc<Type>,
    },
    Var {
        span: Span<'src>,
        var: Var<'src>,
    },
    Call {
        span: Span<'src>,
        name: String,
        args: Vec<Expr<'src>>,
        ty: Arc<Type>,
    },
    Expr {
        expr: Box<Expr<'src>>,
    },
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::Value<'src>> for Value<'src> {
    async fn check0(ctx: Dep<'_, 'src>, value: ast::Value<'src>) -> miette::Result<Self>
    where
        Self: Sized,
    {
        match value {
            ast::Value::CBool { span, value } => {
                Ok(Value::Const {
                    span,
                    value: if value { 1 } else { 0 },
                    ty: Bool.ty(),
                })
            }
            ast::Value::CChar { span, value } => {
                Ok(Value::Const {
                    span,
                    value: value as i32,
                    ty: Char.ty(),
                })
            }
            ast::Value::CNum { span, value } => {
                Ok(Value::Const {
                    span,
                    value,
                    ty: NumConst.ty(),
                })
            }
            ast::Value::Var { span, name } => {
                Ok(Value::Var {
                    span,
                    var: var_find(&mut ctx.var, &name).await?,
                })
            }
            ast::Value::Call { span, ident, args } => {
                // 実引数を順に意味解析
                let mut checked_args = vec![];
                for arg in args {
                    checked_args.push(Expr::check(ctx, arg).await?);
                }

                // 型チェック
                let mod_name = ctx.name.as_str().split(".").collect::<Vec<_>>()[1];
                let fn_name = format!(".{}.{}", mod_name, ident.as_str());
                let fn_ty = TypeDeclChecker::find(&ctx.type_decl, &fn_name).await?;
                let ty = ty_can_call(&fn_ty, &checked_args)?;

                Ok(Value::Call {
                    span,
                    name: fn_name,
                    args: checked_args,
                    ty,
                })
            }
            ast::Value::Expr { expr } => {
                Ok(Value::Expr {
                    expr: Box::new(Expr::check(ctx, *expr).await?),
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
            Value::Call { span, .. } => *span,
            Value::Expr { expr } => expr.span(),
        }
    }
}

impl Typed for Value<'_> {
    fn ty(&self) -> Arc<Type> {
        match self {
            Value::Const { ty, .. } => ty.ty(),
            Value::Var { var, .. } => var.ty.ty(),
            Value::Call { ty, .. } => ty.ty(),
            Value::Expr { expr, .. } => expr.ty(),
        }
    }
}
