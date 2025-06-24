use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_type::op::ty_equals;
use sb_compiler_type::r#type::{Bool, Primitive, Type};
use sb_compiler_type::Typed;

use super::{BitShift, SemCheck, Dep};

#[derive(Debug)]
pub enum Cond<'src> {
    Eq {
        span: Span<'src>,
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
        ty: Arc<Type>,
    },
    Neq {
        span: Span<'src>,
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
        ty: Arc<Type>,
    },
    Lt {
        span: Span<'src>,
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
        ty: Arc<Type>,
    },
    Lte {
        span: Span<'src>,
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
        ty: Arc<Type>,
    },
    Gt {
        span: Span<'src>,
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
        ty: Arc<Type>,
    },
    Gte {
        span: Span<'src>,
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
        ty: Arc<Type>,
    },
    BitShift {
        bit_shift: BitShift<'src>,
    },
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::Cond<'src>> for Cond<'src> {
    async fn check0(ctx: Dep<'_, 'src>, cond: ast::Cond<'src>) -> miette::Result<Self>
    where
        Self: Sized,
    {
        match cond {
            ast::Cond::Eq { span, lhs, rhs } => {
                // 式の意味解析 & 型チェック
                let lhs = Box::new(Cond::check(ctx, *lhs).await?);
                let rhs = BitShift::check(ctx, rhs).await?;
                ty_equals(lhs.ty(), rhs.ty())?;

                // == の型は Bool
                let ty = Arc::new(Primitive(Bool));

                Ok(Cond::Eq { span, lhs, rhs, ty })
            }
            ast::Cond::Neq { span, lhs, rhs } => {
                // 式の意味解析 & 型チェック
                let lhs = Box::new(Cond::check(ctx, *lhs).await?);
                let rhs = BitShift::check(ctx, rhs).await?;
                ty_equals(lhs.ty(), rhs.ty())?;

                // != の型は Bool
                let ty = Arc::new(Primitive(Bool));

                Ok(Cond::Neq { span, lhs, rhs, ty })
            }
            ast::Cond::Lt { span, lhs, rhs } => {
                // 式の意味解析 & 型チェック
                let lhs = Box::new(Cond::check(ctx, *lhs).await?);
                let rhs = BitShift::check(ctx, rhs).await?;
                ty_equals(lhs.ty(), rhs.ty())?;

                // < の型は Bool
                let ty = Arc::new(Primitive(Bool));

                Ok(Cond::Lt { span, lhs, rhs, ty })
            }
            ast::Cond::Lte { span, lhs, rhs } => {
                // 式の意味解析 & 型チェック
                let lhs = Box::new(Cond::check(ctx, *lhs).await?);
                let rhs = BitShift::check(ctx, rhs).await?;
                ty_equals(lhs.ty(), rhs.ty())?;

                // <= の型は Bool
                let ty = Arc::new(Primitive(Bool));

                Ok(Cond::Lte { span, lhs, rhs, ty })
            }
            ast::Cond::Gt { span, lhs, rhs } => {
                // 式の意味解析 & 型チェック
                let lhs = Box::new(Cond::check(ctx, *lhs).await?);
                let rhs = BitShift::check(ctx, rhs).await?;
                ty_equals(lhs.ty(), rhs.ty())?;

                // > の型は Bool
                let ty = Arc::new(Primitive(Bool));

                Ok(Cond::Gt { span, lhs, rhs, ty })
            }
            ast::Cond::Gte { span, lhs, rhs } => {
                // 式の意味解析 & 型チェック
                let lhs = Box::new(Cond::check(ctx, *lhs).await?);
                let rhs = BitShift::check(ctx, rhs).await?;
                ty_equals(lhs.ty(), rhs.ty())?;

                // >= の型は bool
                let ty = Arc::new(Primitive(Bool));

                Ok(Cond::Gte { span, lhs, rhs, ty })
            }
            ast::Cond::BitShift { bit_shift } => {
                Ok(Cond::BitShift {
                    bit_shift: BitShift::check(ctx, bit_shift).await?,
                })
            }
        }
    }
}

impl<'src> Spanned<'src> for Cond<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            Cond::Eq { span, .. } => *span,
            Cond::Neq { span, .. } => *span,
            Cond::Lt { span, .. } => *span,
            Cond::Lte { span, .. } => *span,
            Cond::Gt { span, .. } => *span,
            Cond::Gte { span, .. } => *span,
            Cond::BitShift { bit_shift } => bit_shift.span(),
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
