use sb_compiler_parse_ast as ast;
use sb_compiler_semcheck_impl_type_decl::Type;

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
                let lhs = Box::new(BitShift::check(ctx, *lhs).await?);
                let rhs = Add::check(ctx, rhs).await?;
                let ty = *lhs.ty();

                Ok(BitShift::L { lhs, rhs, ty })
            }
            ast::BitShift::R { lhs, rhs } => {
                let lhs = Box::new(BitShift::check(ctx, *lhs).await?);
                let rhs = Add::check(ctx, rhs).await?;
                let ty = *lhs.ty();

                Ok(BitShift::R { lhs, rhs, ty })
            }
            ast::BitShift::Ra { lhs, rhs } => {
                let lhs = Box::new(BitShift::check(ctx, *lhs).await?);
                let rhs = Add::check(ctx, rhs).await?;
                let ty = *lhs.ty();

                Ok(BitShift::Ra { lhs, rhs, ty })
            }
            ast::BitShift::Add { add } => {
                Ok(BitShift::Add {
                    add: Add::check(ctx, add).await?,
                })
            }
        }
    }

    fn ty(&self) -> &Type {
        match self {
            BitShift::L { ty, .. } => ty,
            BitShift::R { ty, .. } => ty,
            BitShift::Ra { ty, .. } => ty,
            BitShift::Add { add } => add.ty(),
        }
    }
}
