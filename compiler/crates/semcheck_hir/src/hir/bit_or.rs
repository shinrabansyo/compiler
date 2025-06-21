use sb_compiler_parse_ast as ast;

use super::{BitXor, SemCheckFrom, Dep};

#[derive(Debug)]
pub enum BitOr<'input> {
    Or {
        lhs: Box<BitOr<'input>>,
        rhs: BitXor<'input>,
    },
    BitXor {
        xor: BitXor<'input>,
    },
}

impl<'input> SemCheckFrom<Dep<'_, 'input>, ast::BitOr<'input>> for BitOr<'input> {
    async fn check0(ctx: Dep<'_, 'input>, or: ast::BitOr<'input>) -> anyhow::Result<Self>
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
