use sb_compiler_parse_ast as ast;
use sb_compiler_type::op::ty_infer2;
use sb_compiler_type::r#type::Type;
use sb_compiler_type::Typed;

use super::{Add, SemCheck, Dep};

#[derive(Debug)]
pub enum BitShift<'src> {
    L {
        lhs: Box<BitShift<'src>>,
        rhs: Add<'src>,
        ty: Type,
    },
    R {
        lhs: Box<BitShift<'src>>,
        rhs: Add<'src>,
        ty: Type,
    },
    Ra {
        lhs: Box<BitShift<'src>>,
        rhs: Add<'src>,
        ty: Type,
    },
    Add {
        add: Add<'src>,
    },
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::BitShift<'src>> for BitShift<'src> {
    async fn check0(ctx: Dep<'_, 'src>, shift: ast::BitShift<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match shift {
            ast::BitShift::L { lhs, rhs } => {
                // 両辺の式の意味解析
                let lhs = Box::new(BitShift::check(ctx, *lhs).await?);
                let rhs = Add::check(ctx, rhs).await?;

                // 型決定
                let ty = ty_infer2(lhs.ty(), rhs.ty())?;

                Ok(BitShift::L { lhs, rhs, ty })
            }
            ast::BitShift::R { lhs, rhs } => {
                // 両辺の式の意味解析
                let lhs = Box::new(BitShift::check(ctx, *lhs).await?);
                let rhs = Add::check(ctx, rhs).await?;

                // 型決定
                let ty = ty_infer2(lhs.ty(), rhs.ty())?;

                Ok(BitShift::R { lhs, rhs, ty })
            }
            ast::BitShift::Ra { lhs, rhs } => {
                // 両辺の式の意味解析
                let lhs = Box::new(BitShift::check(ctx, *lhs).await?);
                let rhs = Add::check(ctx, rhs).await?;

                // 型決定
                let ty = ty_infer2(lhs.ty(), rhs.ty())?;

                Ok(BitShift::Ra { lhs, rhs, ty })
            }
            ast::BitShift::Add { add } => {
                Ok(BitShift::Add {
                    add: Add::check(ctx, add).await?,
                })
            }
        }
    }
}

impl Typed for BitShift<'_> {
    fn ty(&self) -> &Type {
        match self {
            BitShift::L { ty, .. } => ty,
            BitShift::R { ty, .. } => ty,
            BitShift::Ra { ty, .. } => ty,
            BitShift::Add { add } => add.ty(),
        }
    }
}
