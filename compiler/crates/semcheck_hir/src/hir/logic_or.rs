use sb_compiler_parse_ast as ast;

use super::{LogicAnd, SemCheckFrom, Dep};

#[derive(Debug)]
pub enum LogicOr<'src> {
    Or {
        lhs: Box<LogicOr<'src>>,
        rhs: LogicAnd<'src>,
    },
    LogicAnd {
        and: LogicAnd<'src>,
    },
}

impl<'src> SemCheckFrom<Dep<'_, 'src>, ast::LogicOr<'src>> for LogicOr<'src> {
    async fn check0(ctx: Dep<'_, 'src>, or: ast::LogicOr<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match or {
            ast::LogicOr::Or { lhs, rhs } => {
                Ok(LogicOr::Or {
                    lhs: Box::new(LogicOr::check(ctx, *lhs).await?),
                    rhs: LogicAnd::check(ctx, rhs).await?,
                })
            }
            ast::LogicOr::LogicAnd { and } => {
                Ok(LogicOr::LogicAnd {
                    and: LogicAnd::check(ctx, and).await?,
                })
            }
        }
    }
}
