use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_type::r#type::{Bool, Primitive, Type};
use sb_compiler_type::Typed;

use super::{BitOr, SemCheck, Dep};

#[derive(Debug)]
pub enum LogicAnd<'src> {
    And {
        span: Span<'src>,
        lhs: Box<LogicAnd<'src>>,
        rhs: BitOr<'src>,
        ty: Arc<Type>,
    },
    BitOr {
        or: BitOr<'src>,
    },
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::LogicAnd<'src>> for LogicAnd<'src> {
    async fn check0(ctx: Dep<'_, 'src>, and: ast::LogicAnd<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match and {
            ast::LogicAnd::And { span, lhs, rhs } => {
                // 両辺の式の意味解析
                let lhs = Box::new(LogicAnd::check(ctx, *lhs).await?);
                let rhs = BitOr::check(ctx, rhs).await?;

                // && の型は bool
                let ty = Arc::new(Primitive(Bool));

                Ok(LogicAnd::And { span, lhs, rhs, ty })
            }
            ast::LogicAnd::BitOr { or } => {
                Ok(LogicAnd::BitOr {
                    or: BitOr::check(ctx, or).await?,
                })
            }
        }
    }
}

impl<'src> Spanned<'src> for LogicAnd<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            LogicAnd::And { span, .. } => *span,
            LogicAnd::BitOr { or } => or.span(),
        }
    }
}

impl Typed for LogicAnd<'_> {
    fn ty(&self) -> &Arc<Type> {
        match self {
            LogicAnd::And { ty, .. } => ty,
            LogicAnd::BitOr { or } => or.ty(),
        }
    }
}
