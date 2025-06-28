use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_type::op::ty_equals_arith2;
use sb_compiler_type::r#type::{Bool, Type};
use sb_compiler_type::Typed;

use super::{BitShift, SemCheck, Dep};

#[derive(Debug)]
pub enum Cond<'src> {
    Eq {
        span: Span<'src>,
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
    },
    Neq {
        span: Span<'src>,
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
    },
    Lt {
        span: Span<'src>,
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
    },
    Lte {
        span: Span<'src>,
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
    },
    Gt {
        span: Span<'src>,
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
    },
    Gte {
        span: Span<'src>,
        lhs: Box<Cond<'src>>,
        rhs: BitShift<'src>,
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
                ty_equals_arith2(&lhs, &rhs)?;

                Ok(Cond::Eq { span, lhs, rhs })
            }
            ast::Cond::Neq { span, lhs, rhs } => {
                // 式の意味解析 & 型チェック
                let lhs = Box::new(Cond::check(ctx, *lhs).await?);
                let rhs = BitShift::check(ctx, rhs).await?;
                ty_equals_arith2(&lhs, &rhs)?;

                Ok(Cond::Neq { span, lhs, rhs })
            }
            ast::Cond::Lt { span, lhs, rhs } => {
                // 式の意味解析 & 型チェック
                let lhs = Box::new(Cond::check(ctx, *lhs).await?);
                let rhs = BitShift::check(ctx, rhs).await?;
                ty_equals_arith2(&lhs, &rhs)?;

                Ok(Cond::Lt { span, lhs, rhs })
            }
            ast::Cond::Lte { span, lhs, rhs } => {
                // 式の意味解析 & 型チェック
                let lhs = Box::new(Cond::check(ctx, *lhs).await?);
                let rhs = BitShift::check(ctx, rhs).await?;
                ty_equals_arith2(&lhs, &rhs)?;

                Ok(Cond::Lte { span, lhs, rhs })
            }
            ast::Cond::Gt { span, lhs, rhs } => {
                // 式の意味解析 & 型チェック
                let lhs = Box::new(Cond::check(ctx, *lhs).await?);
                let rhs = BitShift::check(ctx, rhs).await?;
                ty_equals_arith2(&lhs, &rhs)?;

                Ok(Cond::Gt { span, lhs, rhs })
            }
            ast::Cond::Gte { span, lhs, rhs } => {
                // 式の意味解析 & 型チェック
                let lhs = Box::new(Cond::check(ctx, *lhs).await?);
                let rhs = BitShift::check(ctx, rhs).await?;
                ty_equals_arith2(&lhs, &rhs)?;

                Ok(Cond::Gte { span, lhs, rhs })
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
    fn ty(&self) -> Arc<Type> {
        match self {
            Cond::BitShift { bit_shift } => bit_shift.ty(),
            _ => Bool.ty(),
        }
    }
}

