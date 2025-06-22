use sb_compiler_parse_ast as ast;
use sb_compiler_type::{Type, Typed};

use super::{BitAnd, SemCheck, Dep};

#[derive(Debug)]
pub enum BitXor<'src> {
    Xor {
        lhs: Box<BitXor<'src>>,
        rhs: BitAnd<'src>,
        ty: Type,
    },
    BitAnd {
        and: BitAnd<'src>,
    },
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::BitXor<'src>> for BitXor<'src> {
    async fn check0(ctx: Dep<'_, 'src>, xor: ast::BitXor<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match xor {
            ast::BitXor::Xor { lhs, rhs } => {
                let lhs = Box::new(BitXor::check(ctx, *lhs).await?);
                let rhs = BitAnd::check(ctx, rhs).await?;
                let ty = *lhs.ty();

                Ok(BitXor::Xor { lhs, rhs, ty })
            }
            ast::BitXor::BitAnd { and } => {
                Ok(BitXor::BitAnd {
                    and: BitAnd::check(ctx, and).await?,
                })
            }
        }
    }
}

impl Typed for BitXor<'_> {
    fn ty(&self) -> &Type {
        match self {
            BitXor::Xor { ty, .. } => ty,
            BitXor::BitAnd { and } => and.ty(),
        }
    }
}
