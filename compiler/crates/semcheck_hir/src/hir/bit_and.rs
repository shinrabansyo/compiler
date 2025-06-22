use sb_compiler_parse_ast as ast;
use sb_compiler_semcheck_impl_type_decl::Type;

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
                let lhs = Box::new(BitAnd::check(ctx, *lhs).await?);
                let rhs = Cond::check(ctx, rhs).await?;
                let ty = *lhs.ty();

                Ok(BitAnd::And { lhs, rhs, ty })
            }
            ast::BitAnd::Cond { cond } => {
                Ok(BitAnd::Cond {
                    cond: Cond::check(ctx, cond).await?,
                })
            }
        }
    }

    fn ty(&self) -> &Type {
        match self {
            BitAnd::And { ty, .. } => ty,
            BitAnd::Cond { cond } => cond.ty(),
        }
    }
}
