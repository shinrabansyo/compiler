use sb_compiler_parse_ast as ast;

use super::{BitOr, SemCheckFrom, Dep};

#[derive(Debug)]
pub enum LogicAnd<'src> {
    And {
        lhs: Box<LogicAnd<'src>>,
        rhs: BitOr<'src>,
    },
    BitOr {
        or: BitOr<'src>,
    },
}

impl<'src> SemCheckFrom<Dep<'_, 'src>, ast::LogicAnd<'src>> for LogicAnd<'src> {
    async fn check0(ctx: Dep<'_, 'src>, and: ast::LogicAnd<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match and {
            ast::LogicAnd::And { lhs, rhs } => {
                Ok(LogicAnd::And {
                    lhs: Box::new(LogicAnd::check(ctx, *lhs).await?),
                    rhs: BitOr::check(ctx, rhs).await?,
                })
            }
            ast::LogicAnd::BitOr { or } => {
                Ok(LogicAnd::BitOr {
                    or: BitOr::check(ctx, or).await?,
                })
            }
        }
    }
}
