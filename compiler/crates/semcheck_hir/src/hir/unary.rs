use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::Span;
use sb_compiler_type::op::ty_equals;
use sb_compiler_type::r#type::{Bool, Primitive, Type};
use sb_compiler_type::Typed;

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
    async fn check0(ctx: Dep<'_, 'src>, unary: ast::Unary<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match unary {
            ast::Unary::Not { span, value } => {
                // 式の意味解析 & 型チェック
                let value = Value::check(ctx, value).await?;
                ty_equals(value.ty(), &Arc::new(Primitive(Bool)))?;

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

impl Typed for Unary<'_> {
    fn ty(&self) -> &Arc<Type> {
        match self {
            Unary::Not { value, .. } => value.ty(),
            Unary::Plus { value, .. } => value.ty(),
            Unary::Minus { value, .. } => value.ty(),
            Unary::Value { value, .. } => value.ty(),
        }
    }
}
