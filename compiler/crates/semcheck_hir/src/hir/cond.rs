use sb_compiler_parse_ast as ast;
use sb_compiler_type::r#type::Type;
use sb_compiler_type::Typed;

use super::{BitShift, SemCheck, Dep};

#[derive(Debug)]
pub enum Cond<'src> {
    Eq {
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
        ty: Type,
    },
    Neq {
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
        ty: Type,
    },
    Lt {
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
        ty: Type,
    },
    Lte {
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
        ty: Type,
    },
    Gt {
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
        ty: Type,
    },
    Gte {
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
        ty: Type,
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
                let lhs = Box::new(Cond::check(ctx, *lhs).await?);
                let rhs = BitShift::check(ctx, rhs).await?;
                let ty = *lhs.ty();

                Ok(Cond::Eq { lhs, rhs, ty })
            }
            ast::Cond::Neq { lhs, rhs } => {
                let lhs = Box::new(Cond::check(ctx, *lhs).await?);
                let rhs = BitShift::check(ctx, rhs).await?;
                let ty = *lhs.ty();

                Ok(Cond::Neq { lhs, rhs, ty })
            }
            ast::Cond::Lt { lhs, rhs } => {
                let lhs = Box::new(Cond::check(ctx, *lhs).await?);
                let rhs = BitShift::check(ctx, rhs).await?;
                let ty = *lhs.ty();

                Ok(Cond::Lt { lhs, rhs, ty })
            }
            ast::Cond::Lte { lhs, rhs } => {
                let lhs = Box::new(Cond::check(ctx, *lhs).await?);
                let rhs = BitShift::check(ctx, rhs).await?;
                let ty = *lhs.ty();

                Ok(Cond::Lte { lhs, rhs, ty })
            }
            ast::Cond::Gt { lhs, rhs } => {
                let lhs = Box::new(Cond::check(ctx, *lhs).await?);
                let rhs = BitShift::check(ctx, rhs).await?;
                let ty = *lhs.ty();

                Ok(Cond::Gt { lhs, rhs, ty })
            }
            ast::Cond::Gte { lhs, rhs } => {
                let lhs = Box::new(Cond::check(ctx, *lhs).await?);
                let rhs = BitShift::check(ctx, rhs).await?;
                let ty = *lhs.ty();

                Ok(Cond::Gte { lhs, rhs, ty })
            }
            ast::Cond::BitShift { bit_shift } => {
                Ok(Cond::BitShift {
                    bit_shift: BitShift::check(ctx, bit_shift).await?,
                })
            }
        }
    }
}

impl Typed for Cond<'_> {
    fn ty(&self) -> &Type {
        match self {
            Cond::Eq { ty, .. } => ty,
            Cond::Neq { ty, .. } => ty,
            Cond::Lt { ty, .. } => ty,
            Cond::Lte { ty, .. } => ty,
            Cond::Gt { ty, .. } => ty,
            Cond::Gte { ty, .. } => ty,
            Cond::BitShift { bit_shift } => bit_shift.ty(),
        }
    }
}
