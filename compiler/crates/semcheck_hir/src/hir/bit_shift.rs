use sb_compiler_parse_ast as ast;

use super::{Add, SemCheckFrom, Dep};

#[derive(Debug)]
pub enum BitShift<'input> {
    L {
        lhs: Box<BitShift<'input>>,
        rhs: Add<'input>,
    },
    R {
        lhs: Box<BitShift<'input>>,
        rhs: Add<'input>,
    },
    Ra {
        lhs: Box<BitShift<'input>>,
        rhs: Add<'input>,
    },
    Add {
        add: Add<'input>,
    },
}

impl<'input> SemCheckFrom<Dep<'_>, ast::BitShift<'input>> for BitShift<'input> {
    async fn check0(ctx: Dep<'_>, shift: ast::BitShift<'input>) -> anyhow::Result<Self>
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
