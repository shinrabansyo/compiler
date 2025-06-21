use sb_compiler_parse_ast as ast;

use super::{Cond, SemCheckFrom, Dep};

#[derive(Debug)]
pub enum BitAnd<'input> {
    And {
        lhs: Box<BitAnd<'input>>,
        rhs: Cond<'input>,
    },
    Cond {
        cond: Cond<'input>,
    },
}

impl<'input> SemCheckFrom<Dep<'_>, ast::BitAnd<'input>> for BitAnd<'input> {
    async fn check0(ctx: Dep<'_>, and: ast::BitAnd<'input>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match and {
            ast::BitAnd::And { lhs, rhs } => {
                Ok(BitAnd::And {
                    lhs: Box::new(BitAnd::check(ctx, *lhs).await?),
                    rhs: Cond::check(ctx, rhs).await?,
                })
            }
            ast::BitAnd::Cond { cond } => {
                Ok(BitAnd::Cond {
                    cond: Cond::check(ctx, cond).await?,
                })
            }
        }
    }
}
