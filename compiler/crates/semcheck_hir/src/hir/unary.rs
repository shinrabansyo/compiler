use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_semcheck_impl_type::parse::ty_parse_type;
use sb_compiler_semcheck_impl_type::op::ty_equals;
use sb_compiler_semcheck_impl_type::{Typed, Type, Bool, I32};

use super::{Value, SemCheck, Dep};

#[derive(Debug)]
pub enum Unary<'src> {
    Not {
        span: Span<'src>,
        value: Value<'src>,
    },
    Plus {
        span: Span<'src>,
        value: Value<'src>,
    },
    Minus {
        span: Span<'src>,
        value: Value<'src>,
    },
    SizeOf {
        span: Span<'src>,
        size: u32,
    },
    Value {
        value: Value<'src>
    },
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::Unary<'src>> for Unary<'src> {
    async fn check0(ctx: Dep<'_, 'src>, unary: ast::Unary<'src>) -> miette::Result<Self>
    where
        Self: Sized,
    {
        match unary {
            ast::Unary::Not { span, value } => {
                // 式の意味解析 & 型チェック
                let value = Value::check(ctx, value).await?;
                ty_equals(&Bool, &value)?;

                Ok(Unary::Not { span, value })
            }
            ast::Unary::Plus { span, value } => {
                Ok(Unary::Plus {
                    span,
                    value: Value::check(ctx, value).await?,
                })
            }
            ast::Unary::Minus { span, value } => {
                Ok(Unary::Minus {
                    span,
                    value: Value::check(ctx, value).await?,
                })
            }
            ast::Unary::SizeOf { span, ty } => {
                Ok(Unary::SizeOf {
                    span,
                    size: ty_parse_type(&ctx.r#type, &ty).await?.size(),
                })
            }
            ast::Unary::Value { value } => {
                Ok(Unary::Value {
                    value: Value::check(ctx, value).await?,
                })
            }
        }
    }
}

impl<'src> Spanned<'src> for Unary<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            Unary::Not { span, .. } => *span,
            Unary::Plus { span, .. } => *span,
            Unary::Minus { span, .. } => *span,
            Unary::SizeOf { span, .. } => *span,
            Unary::Value { value, .. } => value.span(),
        }
    }
}

impl Typed for Unary<'_> {
    fn ty(&self) -> Arc<Type> {
        match self {
            Unary::Not { value, .. } => value.ty(),
            Unary::Plus { value, .. } => value.ty(),
            Unary::Minus { value, .. } => value.ty(),
            Unary::SizeOf { .. } => I32.ty(),
            Unary::Value { value, .. } => value.ty(),
        }
    }
}
