use sb_compiler_parse_ast as ast;
use sb_compiler_type::op::ty_infer2;
use sb_compiler_type::r#type::Type;
use sb_compiler_type::Typed;

use super::{Cast, SemCheck, Dep};

#[derive(Debug)]
pub enum Add<'src> {
    Plus {
        lhs: Box<Add<'src>>,
        rhs: Cast<'src>,
        ty: Type,
    },
    Minus {
        lhs: Box<Add<'src>>,
        rhs: Cast<'src>,
        ty: Type,
    },
    Cast {
        value: Cast<'src>,
    },
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::Add<'src>> for Add<'src> {
    async fn check0(ctx: Dep<'_, 'src>, add: ast::Add<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match add {
            ast::Add::Plus { lhs, rhs } => {
                // 両辺の式の意味解析
                let lhs = Box::new(Add::check(ctx, *lhs).await?);
                let rhs = Cast::check(ctx, rhs).await?;

                // 型決定
                let ty = ty_infer2(lhs.ty(), rhs.ty())?;

                Ok(Add::Plus { lhs, rhs, ty })
            }
            ast::Add::Minus { lhs, rhs } => {
                // 両辺の式の意味解析
                let lhs = Box::new(Add::check(ctx, *lhs).await?);
                let rhs = Cast::check(ctx, rhs).await?;

                // 型決定
                let ty = ty_infer2(lhs.ty(), rhs.ty())?;

                Ok(Add::Minus { lhs, rhs, ty })
            }
            ast::Add::Cast { value } => {
                Ok(Add::Cast {
                    value: Cast::check(ctx, value).await?,
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
            Add::Cast { value } => value.ty(),
        }
    }
}
