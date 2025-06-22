use sb_compiler_parse_ast as ast;
use sb_compiler_type::op::ty_infer2;
use sb_compiler_type::r#type::Type;
use sb_compiler_type::Typed;

use super::{Cond, SemCheck, Dep};

#[derive(Debug)]
pub enum BitAnd<'src> {
    And {
        lhs: Box<BitAnd<'src>>,
        rhs: Cond<'src>,
        ty: Type,
    },
    Cond {
        cond: Cond<'src>,
    },
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::BitAnd<'src>> for BitAnd<'src> {
    async fn check0(ctx: Dep<'_, 'src>, and: ast::BitAnd<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match and {
            ast::BitAnd::And { lhs, rhs } => {
                // 両辺の式の意味解析
                let lhs = Box::new(BitAnd::check(ctx, *lhs).await?);
                let rhs = Cond::check(ctx, rhs).await?;

                // 型決定
                let ty = ty_infer2(lhs.ty(), rhs.ty())?;

                Ok(BitAnd::And { lhs, rhs, ty })
            }
            ast::BitAnd::Cond { cond } => {
                Ok(BitAnd::Cond {
                    cond: Cond::check(ctx, cond).await?,
                })
            }
        }
    }
}

impl Typed for BitAnd<'_> {
    fn ty(&self) -> &Type {
        match self {
            BitAnd::And { ty, .. } => ty,
            BitAnd::Cond { cond } => cond.ty(),
        }
    }
}
