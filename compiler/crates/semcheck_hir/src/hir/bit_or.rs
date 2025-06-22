use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_type::op::ty_infer2;
use sb_compiler_type::r#type::Type;
use sb_compiler_type::Typed;

use super::{BitXor, SemCheck, Dep};

#[derive(Debug)]
pub enum BitOr<'src> {
    Or {
        lhs: Box<BitOr<'src>>,
        rhs: BitXor<'src>,
        ty: Arc<Type>,
    },
    BitXor {
        xor: BitXor<'src>,
    },
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::BitOr<'src>> for BitOr<'src> {
    async fn check0(ctx: Dep<'_, 'src>, or: ast::BitOr<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match or {
            ast::BitOr::Or { lhs, rhs } => {
                // 両辺の式の意味解析
                let lhs = Box::new(BitOr::check(ctx, *lhs).await?);
                let rhs = BitXor::check(ctx, rhs).await?;

                // 型決定
                let ty = ty_infer2(lhs.ty(), rhs.ty())?;

                Ok(BitOr::Or { lhs, rhs, ty })
            }
            ast::BitOr::BitXor { xor } => {
                Ok(BitOr::BitXor {
                    xor: BitXor::check(ctx, xor).await?,
                })
            }
        }
    }
}

impl Typed for BitOr<'_> {
    fn ty(&self) -> &Arc<Type> {
        match self {
            BitOr::Or { ty, .. } => ty,
            BitOr::BitXor { xor } => xor.ty(),
        }
    }
}
