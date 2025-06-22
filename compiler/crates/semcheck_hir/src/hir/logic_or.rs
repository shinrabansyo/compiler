use sb_compiler_parse_ast as ast;
use sb_compiler_semcheck_impl_type_decl::Type;

use super::{LogicAnd, SemCheck, Dep};

#[derive(Debug)]
pub enum LogicOr<'src> {
    Or {
        lhs: Box<LogicOr<'src>>,
        rhs: LogicAnd<'src>,
        ty: Type,
    },
    LogicAnd {
        and: LogicAnd<'src>,
    },
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::LogicOr<'src>> for LogicOr<'src> {
    async fn check0(ctx: Dep<'_, 'src>, or: ast::LogicOr<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match or {
            ast::LogicOr::Or { lhs, rhs } => {
                let lhs = Box::new(LogicOr::check(ctx, *lhs).await?);
                let rhs = LogicAnd::check(ctx, rhs).await?;
                let ty = *lhs.ty();

                Ok(LogicOr::Or { lhs, rhs, ty })
            }
            ast::LogicOr::LogicAnd { and } => {
                Ok(LogicOr::LogicAnd {
                    and: LogicAnd::check(ctx, and).await?,
                })
            }
        }
    }

    fn ty(&self) -> &Type {
        match self {
            LogicOr::Or { ty, .. } => ty,
            LogicOr::LogicAnd { and } => and.ty(),
        }
    }
}
