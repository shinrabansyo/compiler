use sb_compiler_parse_ast as ast;

use super::{BitShift, SemCheck, Dep};

#[derive(Debug)]
pub enum Cond<'src> {
    Eq {
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
    },
    Neq {
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
    },
    Lt {
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
    },
    Lte {
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
    },
    Gt {
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
    },
    Gte {
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
    },
    BitShift {
        bit_shift: BitShift<'src>,
    },
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::Cond<'src>> for Cond<'src> {
    async fn check0(ctx: Dep<'_, 'src>, cond: ast::Cond<'src>) -> anyhow::Result<Self>
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
