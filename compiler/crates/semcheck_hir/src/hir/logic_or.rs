use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_type::r#type::{Bool, Type};
use sb_compiler_type::Typed;

use super::{LogicAnd, SemCheck, Dep};

#[derive(Debug)]
pub enum LogicOr<'src> {
    Or {
        span: Span<'src>,
        lhs: Box<LogicOr<'src>>,
        rhs: LogicAnd<'src>,
        ty: Arc<Type>,
    },
    LogicAnd {
        and: LogicAnd<'src>,
    },
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::LogicOr<'src>> for LogicOr<'src> {
    async fn check0(ctx: Dep<'_, 'src>, or: ast::LogicOr<'src>) -> miette::Result<Self>
    where
        Self: Sized,
    {
        match or {
            ast::LogicOr::Or { span, lhs, rhs } => {
                // 両辺の式の意味解析
                let lhs = Box::new(LogicOr::check(ctx, *lhs).await?);
                let rhs = LogicAnd::check(ctx, rhs).await?;

                // || の型は Bool
                let ty = Arc::new(Bool);

                Ok(LogicOr::Or { span, lhs, rhs, ty })
            }
            ast::LogicOr::LogicAnd { and } => {
                Ok(LogicOr::LogicAnd {
                    and: LogicAnd::check(ctx, and).await?,
                })
            }
        }
    }
}

impl<'src> Spanned<'src> for LogicOr<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            LogicOr::Or { span, .. } => *span,
            LogicOr::LogicAnd { and } => and.span(),
        }
    }
}

impl Typed for LogicOr<'_> {
    fn ty(&self) -> &Arc<Type> {
        match self {
            LogicOr::Or { ty, .. } => ty,
            LogicOr::LogicAnd { and } => and.ty(),
        }
    }
}
