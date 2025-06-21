use sb_compiler_parse_ast as ast;

use super::{Unary, SemCheckFrom, Dep};

#[derive(Debug)]
pub enum Add<'src> {
    Plus {
        lhs: Box<Add<'src>>,
        rhs: Unary<'src>,
    },
    Minus {
        lhs: Box<Add<'src>>,
        rhs: Unary<'src>,
    },
    Unary {
        value: Unary<'src>,
    },
}

impl<'src> SemCheckFrom<Dep<'_, 'src>, ast::Add<'src>> for Add<'src> {
    async fn check0(ctx: Dep<'_, 'src>, add: ast::Add<'src>) -> anyhow::Result<Self>
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
