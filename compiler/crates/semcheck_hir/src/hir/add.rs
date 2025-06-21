use sb_compiler_parse_ast as ast;

use super::{Unary, SemCheckFrom, Dep};

#[derive(Debug)]
pub enum Add<'input> {
    Plus {
        lhs: Box<Add<'input>>,
        rhs: Unary<'input>,
    },
    Minus {
        lhs: Box<Add<'input>>,
        rhs: Unary<'input>,
    },
    Unary {
        value: Unary<'input>,
    },
}

impl<'input> SemCheckFrom<Dep<'_, 'input>, ast::Add<'input>> for Add<'input> {
    async fn check0(ctx: Dep<'_, 'input>, add: ast::Add<'input>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match add {
            ast::Add::Plus { lhs, rhs } => {
                Ok(Add::Plus {
                    lhs: Box::new(Add::check(ctx, *lhs).await?),
                    rhs: Unary::check(ctx, rhs).await?,
                })
            }
            ast::Add::Minus { lhs, rhs } => {
                Ok(Add::Minus {
                    lhs: Box::new(Add::check(ctx, *lhs).await?),
                    rhs: Unary::check(ctx, rhs).await?,
                })
            }
            ast::Add::Unary { value } => {
                Ok(Add::Unary {
                    value: Unary::check(ctx, value).await?,
                })
            }
        }
    }
}
