use sb_compiler_parse_ast as ast;

use super::{Value, SemCheckFrom, Dep};

#[derive(Debug)]
pub enum Unary<'input> {
    Plus {
        value: Value<'input>,
    },
    Minus {
        value: Value<'input>,
    },
    Value {
        value: Value<'input>
    },
}

impl<'input> SemCheckFrom<Dep<'_>, ast::Unary<'input>> for Unary<'input> {
    async fn check0(ctx: Dep<'_>, unary: ast::Unary<'input>) -> anyhow::Result<Self>
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
