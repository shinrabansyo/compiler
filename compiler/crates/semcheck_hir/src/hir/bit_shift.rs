use sb_compiler_parse_ast as ast;

use super::{Add, SemCheckFrom, Dep};

#[derive(Debug)]
pub enum BitShift<'src> {
    L {
        lhs: Box<BitShift<'src>>,
        rhs: Add<'src>,
    },
    R {
        lhs: Box<BitShift<'src>>,
        rhs: Add<'src>,
    },
    Ra {
        lhs: Box<BitShift<'src>>,
        rhs: Add<'src>,
    },
    Add {
        add: Add<'src>,
    },
}

impl<'src> SemCheckFrom<Dep<'_, 'src>, ast::BitShift<'src>> for BitShift<'src> {
    async fn check0(ctx: Dep<'_, 'src>, shift: ast::BitShift<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match shift {
            ast::BitShift::L { lhs, rhs } => {
                Ok(BitShift::L {
                    lhs: Box::new(BitShift::check(ctx, *lhs).await?),
                    rhs: Add::check(ctx, rhs).await?,
                })
            }
            ast::BitShift::R { lhs, rhs } => {
                Ok(BitShift::R {
                    lhs: Box::new(BitShift::check(ctx, *lhs).await?),
                    rhs: Add::check(ctx, rhs).await?,
                })
            }
            ast::BitShift::Ra { lhs, rhs } => {
                Ok(BitShift::Ra {
                    lhs: Box::new(BitShift::check(ctx, *lhs).await?),
                    rhs: Add::check(ctx, rhs).await?,
                })
            }
            ast::BitShift::Add { add } => {
                Ok(BitShift::Add {
                    add: Add::check(ctx, add).await?,
                })
            }
        }
    }
}
