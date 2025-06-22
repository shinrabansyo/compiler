use sb_compiler_parse_ast as ast;
use sb_compiler_type::op::ty_infer2;
use sb_compiler_type::r#type::Type;
use sb_compiler_type::Typed;

use super::{BitOr, SemCheck, Dep};

#[derive(Debug)]
pub enum LogicAnd<'src> {
    And {
        lhs: Box<LogicAnd<'src>>,
        rhs: BitOr<'src>,
        ty: Type,
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
            ast::LogicAnd::And { lhs, rhs } => {
                // 両辺の式の意味解析
                let lhs = Box::new(LogicAnd::check(ctx, *lhs).await?);
                let rhs = BitOr::check(ctx, rhs).await?;

                // 型決定
                let ty = ty_infer2(lhs.ty(), rhs.ty())?;

                Ok(LogicAnd::And { lhs, rhs, ty })
            }
            ast::LogicAnd::BitOr { or } => {
                Ok(LogicAnd::BitOr {
                    or: BitOr::check(ctx, or).await?,
                })
            }
        }
    }
}

impl Typed for LogicAnd<'_> {
    fn ty(&self) -> &Type {
        match self {
            LogicAnd::And { ty, .. } => ty,
            LogicAnd::BitOr { or } => or.ty(),
        }
    }
}
