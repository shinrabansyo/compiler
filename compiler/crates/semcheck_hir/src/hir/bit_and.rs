use sb_compiler_parse_ast as ast;

use super::{Cond, SemCheckFrom, Dep};

#[derive(Debug)]
pub enum BitAnd<'src> {
    And {
        lhs: Box<BitAnd<'src>>,
        rhs: Cond<'src>,
    },
    Cond {
        cond: Cond<'src>,
    },
}

impl<'src> SemCheckFrom<Dep<'_, 'src>, ast::BitAnd<'src>> for BitAnd<'src> {
    async fn check0(ctx: Dep<'_, 'src>, and: ast::BitAnd<'src>) -> anyhow::Result<Self>
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
