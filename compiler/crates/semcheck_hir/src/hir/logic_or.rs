use sb_compiler_parse_ast as ast;

use super::{LogicAnd, SemCheckFrom, Dep};

#[derive(Debug)]
pub enum LogicOr<'input> {
    Or {
        lhs: Box<LogicOr<'input>>,
        rhs: LogicAnd<'input>,
    },
    LogicAnd {
        and: LogicAnd<'input>,
    },
}

impl<'input> SemCheckFrom<Dep<'_>, ast::LogicOr<'input>> for LogicOr<'input> {
    async fn check0(ctx: Dep<'_>, or: ast::LogicOr<'input>) -> anyhow::Result<Self>
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
