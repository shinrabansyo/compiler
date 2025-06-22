use sb_compiler_parse_ast as ast;
use sb_compiler_type::{Type, Typed};

use super::{Unary, SemCheck, Dep};

#[derive(Debug)]
pub enum Add<'src> {
    Plus {
        lhs: Box<Add<'src>>,
        rhs: Unary<'src>,
        ty: Type,
    },
    Minus {
        lhs: Box<Add<'src>>,
        rhs: Unary<'src>,
        ty: Type,
    },
    Unary {
        value: Unary<'src>,
    },
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::Add<'src>> for Add<'src> {
    async fn check0(ctx: Dep<'_, 'src>, add: ast::Add<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match add {
            ast::Add::Plus { lhs, rhs } => {
                let lhs = Box::new(Add::check(ctx, *lhs).await?);
                let rhs = Unary::check(ctx, rhs).await?;
                let ty = *lhs.ty();

                Ok(Add::Plus { lhs, rhs, ty })
            }
            ast::Add::Minus { lhs, rhs } => {
                let lhs = Box::new(Add::check(ctx, *lhs).await?);
                let rhs = Unary::check(ctx, rhs).await?;
                let ty = *lhs.ty();

                Ok(Add::Minus { lhs, rhs, ty })
            }
            ast::Add::Unary { value } => {
                Ok(Add::Unary {
                    value: Unary::check(ctx, value).await?,
                })
            }
        }
    }
}

impl Typed for Add<'_> {
    fn ty(&self) -> &Type {
        match self {
            Add::Plus { ty, .. } => ty,
            Add::Minus { ty, .. } => ty,
            Add::Unary { value } => value.ty(),
        }
    }
}
