use sb_compiler_parse_ast as ast;

use super::{BitAnd, SemCheckFrom, Dep};

#[derive(Debug)]
pub enum BitXor<'input> {
    Xor {
        lhs: Box<BitXor<'input>>,
        rhs: BitAnd<'input>,
    },
    BitAnd {
        and: BitAnd<'input>,
    },
}

impl<'input> SemCheckFrom<Dep<'_, 'input>, ast::BitXor<'input>> for BitXor<'input> {
    async fn check0(ctx: Dep<'_, 'input>, xor: ast::BitXor<'input>) -> anyhow::Result<Self>
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
