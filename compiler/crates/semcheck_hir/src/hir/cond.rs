use sb_compiler_parse_ast as ast;

use super::{BitShift, SemCheckFrom, Dep};

#[derive(Debug)]
pub enum Cond<'input> {
    Eq {
        lhs: Box<Cond<'input>>,
        rhs: BitShift<'input>,
    },
    Neq {
        lhs: Box<Cond<'input>>,
        rhs: BitShift<'input>,
    },
    Lt {
        lhs: Box<Cond<'input>>,
        rhs: BitShift<'input>,
    },
    Lte {
        lhs: Box<Cond<'input>>,
        rhs: BitShift<'input>,
    },
    Gt {
        lhs: Box<Cond<'input>>,
        rhs: BitShift<'input>,
    },
    Gte {
        lhs: Box<Cond<'input>>,
        rhs: BitShift<'input>,
    },
    BitShift {
        bit_shift: BitShift<'input>,
    },
}

impl<'input> SemCheckFrom<Dep<'_, 'input>, ast::Cond<'input>> for Cond<'input> {
    async fn check0(ctx: Dep<'_, 'input>, cond: ast::Cond<'input>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match cond {
            ast::Cond::Eq { lhs, rhs } => {
                Ok(Cond::Eq {
                    lhs: Box::new(Cond::check(ctx, *lhs).await?),
                    rhs: BitShift::check(ctx, rhs).await?,
                })
            }
            ast::Cond::Neq { lhs, rhs } => {
                Ok(Cond::Neq {
                    lhs: Box::new(Cond::check(ctx, *lhs).await?),
                    rhs: BitShift::check(ctx, rhs).await?,
                })
            }
            ast::Cond::Lt { lhs, rhs } => {
                Ok(Cond::Lt {
                    lhs: Box::new(Cond::check(ctx, *lhs).await?),
                    rhs: BitShift::check(ctx, rhs).await?,
                })
            }
            ast::Cond::Lte { lhs, rhs } => {
                Ok(Cond::Lte {
                    lhs: Box::new(Cond::check(ctx, *lhs).await?),
                    rhs: BitShift::check(ctx, rhs).await?,
                })
            }
            ast::Cond::Gt { lhs, rhs } => {
                Ok(Cond::Gt {
                    lhs: Box::new(Cond::check(ctx, *lhs).await?),
                    rhs: BitShift::check(ctx, rhs).await?,
                })
            }
            ast::Cond::Gte { lhs, rhs } => {
                Ok(Cond::Gte {
                    lhs: Box::new(Cond::check(ctx, *lhs).await?),
                    rhs: BitShift::check(ctx, rhs).await?,
                })
            }
            ast::Cond::BitShift { bit_shift } => {
                Ok(Cond::BitShift {
                    bit_shift: BitShift::check(ctx, bit_shift).await?,
                })
            }
        }
    }
}
