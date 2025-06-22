use sb_compiler_parse_ast as ast;
use sb_compiler_type::op::ty_infer2;
use sb_compiler_type::r#type::Type;
use sb_compiler_type::Typed;

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
                // 両辺の式の意味解析
                let lhs = Box::new(LogicOr::check(ctx, *lhs).await?);
                let rhs = LogicAnd::check(ctx, rhs).await?;

                // 型決定
                let ty = ty_infer2(lhs.ty(), rhs.ty())?;

                Ok(LogicOr::Or { lhs, rhs, ty })
            }
            ast::LogicOr::LogicAnd { and } => {
                Ok(LogicOr::LogicAnd {
                    and: LogicAnd::check(ctx, and).await?,
                })
            }
        }
    }
}

impl Typed for LogicOr<'_> {
    fn ty(&self) -> &Type {
        match self {
            LogicOr::Or { ty, .. } => ty,
            LogicOr::LogicAnd { and } => and.ty(),
        }
    }
}
