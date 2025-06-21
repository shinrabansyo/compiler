use sb_compiler_parse_ast as ast;

use super::{BitXor, SemCheckFrom, Dep};

#[derive(Debug)]
pub enum BitOr<'src> {
    Or {
        lhs: Box<BitOr<'src>>,
        rhs: BitXor<'src>,
    },
    BitXor {
        xor: BitXor<'src>,
    },
}

impl<'src> SemCheckFrom<Dep<'_, 'src>, ast::BitOr<'src>> for BitOr<'src> {
    async fn check0(ctx: Dep<'_, 'src>, or: ast::BitOr<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match or {
            ast::BitOr::Or { lhs, rhs } => {
                Ok(BitOr::Or {
                    lhs: Box::new(BitOr::check(ctx, *lhs).await?),
                    rhs: BitXor::check(ctx, rhs).await?,
                })
            }
            ast::BitOr::BitXor { xor } => {
                Ok(BitOr::BitXor {
                    xor: BitXor::check(ctx, xor).await?,
                })
            }
        }
    }
}
