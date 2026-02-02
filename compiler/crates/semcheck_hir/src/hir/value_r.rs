use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_semcheck_impl_var::decl::var_find;
use sb_compiler_semcheck_impl_var::Var;
use sb_compiler_semcheck_impl_type::r#fn::ty_can_call;
use sb_compiler_semcheck_impl_type::{Typed, Type, Bool, Char, NumConst};

use super::{Expr, StructAccess, StructInit, SemCheck, Dep};

#[derive(Debug)]
pub enum ValueR<'src> {
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
    StructInit {
        struct_init: StructInit<'src>,
    },
    StructAccess {
        struct_access: StructAccess<'src>,
    },
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::ValueR<'src>> for ValueR<'src> {
    async fn check0(ctx: Dep<'_, 'src>, value: ast::ValueR<'src>) -> miette::Result<Self>
    where
        Self: Sized,
    {
        match value {
            ast::ValueR::CBool { span, value } => {
                Ok(ValueR::Const {
                    span,
                    value: if value { 1 } else { 0 },
                    ty: Bool.ty(),
                })
            }
            ast::ValueR::CChar { span, value } => {
                Ok(ValueR::Const {
                    span,
                    value: value as i32,
                    ty: Char.ty(),
                })
            }
            ast::ValueR::CNum { span, value } => {
                Ok(ValueR::Const {
                    span,
                    value,
                    ty: NumConst.ty(),
                })
            }
            ast::ValueR::Var { span, name } => {
                Ok(ValueR::Var {
                    span,
                    var: var_find(&mut ctx.var, &name).await?,
                })
            }
            ast::ValueR::Call { span, ident, args } => {
                // 関数名(フルパス)
                let fn_name = format!(".main.{}", ident.as_str());

                // 実引数を順に意味解析
                let mut checked_args = vec![];
                for arg in args {
                    checked_args.push(Expr::check(ctx, arg).await?);
                }

                // 型チェック
                let fn_ty = ty_can_call(&ctx.r#type, ident, &checked_args).await?;

                Ok(ValueR::Call {
                    span,
                    name: fn_name,
                    args: checked_args,
                    ty: fn_ty,
                })
            }
            ast::ValueR::Expr { expr } => {
                Ok(ValueR::Expr {
                    expr: Box::new(Expr::check(ctx, *expr).await?),
                })
            }
            ast::ValueR::StructInit { struct_init } => {
                Ok(ValueR::StructInit {
                    struct_init: StructInit::check(ctx, struct_init).await?,
                })
            }
            ast::ValueR::StructAccess { struct_access } => {
                Ok(ValueR::StructAccess {
                    struct_access: StructAccess::check(ctx, struct_access).await?,
                })
            }
        }
    }
}

impl<'src> Spanned<'src> for ValueR<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            ValueR::Const { span, .. } => *span,
            ValueR::Var { span, .. } => *span,
            ValueR::Call { span, .. } => *span,
            ValueR::Expr { expr } => expr.span(),
            ValueR::StructInit { struct_init } => struct_init.span(),
            ValueR::StructAccess { struct_access } => struct_access.span(),
        }
    }
}

impl Typed for ValueR<'_> {
    fn ty(&self) -> Arc<Type> {
        match self {
            ValueR::Const { ty, .. } => ty.ty(),
            ValueR::Var { var, .. } => var.ty.ty(),
            ValueR::Call { ty, .. } => ty.ty(),
            ValueR::Expr { expr, .. } => expr.ty(),
            ValueR::StructInit { struct_init } => struct_init.ty(),
            ValueR::StructAccess { struct_access } => struct_access.ty(),
        }
    }
}
