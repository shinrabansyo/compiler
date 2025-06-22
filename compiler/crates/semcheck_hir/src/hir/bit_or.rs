use sb_compiler_parse_ast as ast;
use sb_compiler_semcheck_impl_type_decl::Type;

use super::{BitXor, SemCheck, Dep};

#[derive(Debug)]
pub enum BitOr<'src> {
    Or {
        lhs: Box<BitOr<'src>>,
        rhs: BitXor<'src>,
        ty: Type,
    },
    BitXor {
        xor: BitXor<'src>,
    },
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::BitOr<'src>> for BitOr<'src> {
    async fn check0(ctx: Dep<'_, 'src>, or: ast::BitOr<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match or {
            ast::BitOr::Or { lhs, rhs } => {
                let lhs = Box::new(BitOr::check(ctx, *lhs).await?);
                let rhs = BitXor::check(ctx, rhs).await?;
                let ty = *lhs.ty();

                Ok(BitOr::Or { lhs, rhs, ty })
            }
            ast::BitOr::BitXor { xor } => {
                Ok(BitOr::BitXor {
                    xor: BitXor::check(ctx, xor).await?,
                })
            }
        }
    }

    fn ty(&self) -> &Type {
        match self {
            BitOr::Or { ty, .. } => ty,
            BitOr::BitXor { xor } => xor.ty(),
        }
    }
}
