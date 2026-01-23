use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_semcheck_impl_type::op::ty_equals;
use sb_compiler_semcheck_impl_type::{Typed, Type, Bool};

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
            Unary::Value { value, .. } => value.ty(),
        }
    }
}
