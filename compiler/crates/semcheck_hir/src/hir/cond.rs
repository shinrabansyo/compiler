use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_type::r#type::{Bool, Primitive, Type};
use sb_compiler_type::Typed;

use super::{BitShift, SemCheck, Dep};

#[derive(Debug)]
pub enum Cond<'src> {
    Eq {
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
        ty: Arc<Type>,
    },
    Neq {
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
        ty: Arc<Type>,
    },
    Lt {
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
        ty: Arc<Type>,
    },
    Lte {
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
        ty: Arc<Type>,
    },
    Gt {
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
        ty: Arc<Type>,
    },
    Gte {
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
        ty: Arc<Type>,
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
                // 式の意味解析
                let lhs = Box::new(Cond::check(ctx, *lhs).await?);
                let rhs = BitShift::check(ctx, rhs).await?;

                // == の型は Bool
                let ty = Arc::new(Primitive(Bool));

                Ok(Cond::Eq { lhs, rhs, ty })
            }
            ast::Cond::Neq { lhs, rhs } => {
                // 式の意味解析
                let lhs = Box::new(Cond::check(ctx, *lhs).await?);
                let rhs = BitShift::check(ctx, rhs).await?;

                // != の型は Bool
                let ty = Arc::new(Primitive(Bool));

                Ok(Cond::Neq { lhs, rhs, ty })
            }
            ast::Cond::Lt { lhs, rhs } => {
                // 式の意味解析
                let lhs = Box::new(Cond::check(ctx, *lhs).await?);
                let rhs = BitShift::check(ctx, rhs).await?;

                // < の型は Bool
                let ty = Arc::new(Primitive(Bool));

                Ok(Cond::Lt { lhs, rhs, ty })
            }
            ast::Cond::Lte { lhs, rhs } => {
                // 式の意味解析
                let lhs = Box::new(Cond::check(ctx, *lhs).await?);
                let rhs = BitShift::check(ctx, rhs).await?;

                // <= の型は Bool
                let ty = Arc::new(Primitive(Bool));

                Ok(Cond::Lte { lhs, rhs, ty })
            }
            ast::Cond::Gt { lhs, rhs } => {
                // 式の意味解析
                let lhs = Box::new(Cond::check(ctx, *lhs).await?);
                let rhs = BitShift::check(ctx, rhs).await?;

                // > の型は Bool
                let ty = Arc::new(Primitive(Bool));

                Ok(Cond::Gt { lhs, rhs, ty })
            }
            ast::Cond::Gte { lhs, rhs } => {
                // 式の意味解析
                let lhs = Box::new(Cond::check(ctx, *lhs).await?);
                let rhs = BitShift::check(ctx, rhs).await?;

                // >= の型は bool
                let ty = Arc::new(Primitive(Bool));

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
    fn ty(&self) -> &Arc<Type> {
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
