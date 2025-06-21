use sb_compiler_parse_ast as ast;

use super::{BitOr, SemCheckFrom, Dep};

#[derive(Debug)]
pub enum LogicAnd<'input> {
    And {
        lhs: Box<LogicAnd<'input>>,
        rhs: BitOr<'input>,
    },
    BitOr {
        or: BitOr<'input>,
    },
}

impl<'input> SemCheckFrom<Dep<'_>, ast::LogicAnd<'input>> for LogicAnd<'input> {
    async fn check0(ctx: Dep<'_>, and: ast::LogicAnd<'input>) -> anyhow::Result<Self>
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
