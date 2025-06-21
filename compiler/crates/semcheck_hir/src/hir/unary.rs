use sb_compiler_parse_ast as ast;

use super::{Value, SemCheck, Dep};

#[derive(Debug)]
pub enum Unary<'src> {
    Plus {
        value: Value<'src>,
    },
    Minus {
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
            ast::Unary::Plus { value } => {
                Ok(Unary::Plus {
                    value: Value::check(ctx, value).await?,
                })
            }
            ast::Unary::Minus { value } => {
                Ok(Unary::Minus {
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
