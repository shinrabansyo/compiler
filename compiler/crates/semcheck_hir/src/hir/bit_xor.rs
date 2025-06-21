use sb_compiler_parse_ast as ast;

use super::{BitAnd, SemCheck, Dep};

#[derive(Debug)]
pub enum BitXor<'src> {
    Xor {
        lhs: Box<BitXor<'src>>,
        rhs: BitAnd<'src>,
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
                Ok(BitXor::Xor {
                    lhs: Box::new(BitXor::check(ctx, *lhs).await?),
                    rhs: BitAnd::check(ctx, rhs).await?,
                })
            }
            ast::BitXor::BitAnd { and } => {
                Ok(BitXor::BitAnd {
                    and: BitAnd::check(ctx, and).await?,
                })
            }
        }
    }
}
