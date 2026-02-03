use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_semcheck_impl_type::op::ty_equals;
use sb_compiler_semcheck_impl_type::{Typed, Type};

use super::{LogicOr, ValueL, SemCheck, Dep};

#[derive(Debug)]
pub enum Assign<'src> {
    Normal {
        span: Span<'src>,
        lhs: ValueL<'src>,
        assign: Box<Assign<'src>>,
    },
    Plus {
        span: Span<'src>,
        lhs: ValueL<'src>,
        assign: Box<Assign<'src>>,
    },
    Minus {
        span: Span<'src>,
        lhs: ValueL<'src>,
        assign: Box<Assign<'src>>,
    },
    Mul {
        span: Span<'src>,
        lhs: ValueL<'src>,
        assign: Box<Assign<'src>>,
    },
    Div {
        span: Span<'src>,
        lhs: ValueL<'src>,
        assign: Box<Assign<'src>>,
    },
    Mod {
        span: Span<'src>,
        lhs: ValueL<'src>,
        assign: Box<Assign<'src>>,
    },
    ShiftL {
        span: Span<'src>,
        lhs: ValueL<'src>,
        assign: Box<Assign<'src>>,
    },
    ShiftR {
        span: Span<'src>,
        lhs: ValueL<'src>,
        assign: Box<Assign<'src>>,
    },
    ShiftRa {
        span: Span<'src>,
        lhs: ValueL<'src>,
        assign: Box<Assign<'src>>,
    },
    LogicOr {
        or: LogicOr<'src>,
    }
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::Assign<'src>> for Assign<'src> {
    async fn check0(ctx: Dep<'_, 'src>, assign: ast::Assign<'src>) -> miette::Result<Self>
    where
        Self: Sized,
    {
        match assign {
            ast::Assign::Normal { span, lhs, assign } => {
                // 両辺の意味解析
                let lhs = ValueL::check(ctx, lhs).await?;
                let assign = Box::new(Assign::check(ctx, *assign).await?);

                // 型チェック
                ty_equals(&lhs.ty(), &assign)?;

                Ok(Assign::Normal { span, lhs, assign })
            }
            ast::Assign::Plus { span, lhs, assign } => {
                // 式の意味解析
                let lhs = ValueL::check(ctx, lhs).await?;
                let assign = Box::new(Assign::check(ctx, *assign).await?);

                // 型チェック
                ty_equals(&lhs.ty(), &assign)?;

                Ok(Assign::Plus { span, lhs, assign })
            }
            ast::Assign::Minus { span, lhs, assign } => {
                // 式の意味解析
                let lhs = ValueL::check(ctx, lhs).await?;
                let assign = Box::new(Assign::check(ctx, *assign).await?);

                // 型チェック
                ty_equals(&lhs.ty(), &assign)?;

                Ok(Assign::Minus { span, lhs, assign })
            }
            ast::Assign::Mul { span, lhs, assign } => {
                // 式の意味解析
                let lhs = ValueL::check(ctx, lhs).await?;
                let assign = Box::new(Assign::check(ctx, *assign).await?);

                // 型チェック
                ty_equals(&lhs.ty(), &assign)?;

                Ok(Assign::Mul { span, lhs, assign })
            }
            ast::Assign::Div { span, lhs, assign } => {
                // 式の意味解析
                let lhs = ValueL::check(ctx, lhs).await?;
                let assign = Box::new(Assign::check(ctx, *assign).await?);

                // 型チェック
                ty_equals(&lhs.ty(), &assign)?;

                Ok(Assign::Div { span, lhs, assign })
            }
            ast::Assign::Mod { span, lhs, assign } => {
                // 式の意味解析
                let lhs = ValueL::check(ctx, lhs).await?;
                let assign = Box::new(Assign::check(ctx, *assign).await?);

                // 型チェック
                ty_equals(&lhs.ty(), &assign)?;

                Ok(Assign::Mod { span, lhs, assign })
            }
            ast::Assign::ShiftL { span, lhs, assign } => {
                // 式の意味解析
                let lhs = ValueL::check(ctx, lhs).await?;
                let assign = Box::new(Assign::check(ctx, *assign).await?);

                // 型チェック
                ty_equals(&lhs.ty(), &assign)?;

                Ok(Assign::ShiftL { span, lhs, assign })
            }
            ast::Assign::ShiftR { span, lhs, assign } => {
                // 式の意味解析
                let lhs = ValueL::check(ctx, lhs).await?;
                let assign = Box::new(Assign::check(ctx, *assign).await?);

                // 型チェック
                ty_equals(&lhs.ty(), &assign)?;

                Ok(Assign::ShiftR { span, lhs, assign })
            }
            ast::Assign::ShiftRa { span, lhs, assign } => {
                // 式の意味解析
                let lhs = ValueL::check(ctx, lhs).await?;
                let assign = Box::new(Assign::check(ctx, *assign).await?);

                // 型チェック
                ty_equals(&lhs.ty(), &assign)?;

                Ok(Assign::ShiftRa{ span, lhs, assign })
            }
            ast::Assign::LogicOr { or } => {
                Ok(Assign::LogicOr {
                    or: LogicOr::check(ctx, or).await?,
                })
            }
        }
    }
}

impl<'src> Spanned<'src> for Assign<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            Assign::Normal { span, .. } => *span,
            Assign::Plus { span, .. } => *span,
            Assign::Minus { span, .. } => *span,
            Assign::Mul { span, .. } => *span,
            Assign::Div { span, .. } => *span,
            Assign::Mod { span, .. } => *span,
            Assign::ShiftL { span, .. } => *span,
            Assign::ShiftR { span, .. } => *span,
            Assign::ShiftRa { span, .. } => *span,
            Assign::LogicOr { or } => or.span(),
        }
    }
}

impl Typed for Assign<'_> {
    fn ty(&self) -> Arc<Type> {
        match self {
            Assign::Normal { lhs, .. } => lhs.ty(),
            Assign::Plus { lhs, .. } => lhs.ty(),
            Assign::Minus { lhs, .. } => lhs.ty(),
            Assign::Mul { lhs, .. } => lhs.ty(),
            Assign::Div { lhs, .. } => lhs.ty(),
            Assign::Mod { lhs, .. } => lhs.ty(),
            Assign::ShiftL { lhs, .. } => lhs.ty(),
            Assign::ShiftR { lhs, .. } => lhs.ty(),
            Assign::ShiftRa { lhs, .. } => lhs.ty(),
            Assign::LogicOr { or } => or.ty(),
        }
    }
}
