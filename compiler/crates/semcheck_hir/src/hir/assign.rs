use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::Span;
use sb_compiler_semcheck_impl_vardecl::{Var, VarDeclChecker};
use sb_compiler_type::op::ty_equals;
use sb_compiler_type::r#type::Type;
use sb_compiler_type::Typed;

use super::{LogicOr, SemCheck, Dep};

#[derive(Debug)]
pub enum Assign<'src> {
    Normal {
        span: Span<'src>,
        var: Var<'src>,
        assign: Box<Assign<'src>>,
    },
    Plus {
        span: Span<'src>,
        var: Var<'src>,
        assign: Box<Assign<'src>>,
    },
    Minus {
        span: Span<'src>,
        var: Var<'src>,
        assign: Box<Assign<'src>>,
    },
    ShiftL {
        span: Span<'src>,
        var: Var<'src>,
        assign: Box<Assign<'src>>,
    },
    ShiftR {
        span: Span<'src>,
        var: Var<'src>,
        assign: Box<Assign<'src>>,
    },
    ShiftRa {
        span: Span<'src>,
        var: Var<'src>,
        assign: Box<Assign<'src>>,
    },
    LogicOr {
        or: LogicOr<'src>,
    }
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::Assign<'src>> for Assign<'src> {
    async fn check0(ctx: Dep<'_, 'src>, assign: ast::Assign<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match assign {
            ast::Assign::Normal { span, ident, assign } => {
                // 式の意味解析
                let assign = Box::new(Assign::check(ctx, *assign).await?);

                // 型チェック
                let var = VarDeclChecker::find(&mut ctx.var_decl, &ident).await?;
                ty_equals(&var.ty, assign.ty())?;

                Ok(Assign::Normal { span, var, assign })
            }
            ast::Assign::Plus { span, ident, assign } => {
                // 式の意味解析
                let assign = Box::new(Assign::check(ctx, *assign).await?);

                // 型チェック
                let var = VarDeclChecker::find(&mut ctx.var_decl, &ident).await?;
                ty_equals(&var.ty, assign.ty())?;

                Ok(Assign::Plus { span, var, assign })
            }
            ast::Assign::Minus { span, ident, assign } => {
                // 式の意味解析
                let assign = Box::new(Assign::check(ctx, *assign).await?);

                // 型チェック
                let var = VarDeclChecker::find(&mut ctx.var_decl, &ident).await?;
                ty_equals(&var.ty, assign.ty())?;

                Ok(Assign::Minus { span, var, assign })
            }
            ast::Assign::ShiftL { span, ident, assign } => {
                // 式の意味解析
                let assign = Box::new(Assign::check(ctx, *assign).await?);

                // 型チェック
                let var = VarDeclChecker::find(&mut ctx.var_decl, &ident).await?;
                ty_equals(&var.ty, assign.ty())?;

                Ok(Assign::ShiftL { span, var, assign })
            }
            ast::Assign::ShiftR { span, ident, assign } => {
                // 式の意味解析
                let assign = Box::new(Assign::check(ctx, *assign).await?);

                // 型チェック
                let var = VarDeclChecker::find(&mut ctx.var_decl, &ident).await?;
                ty_equals(&var.ty, assign.ty())?;

                Ok(Assign::ShiftR { span, var, assign })
            }
            ast::Assign::ShiftRa { span, ident, assign } => {
                // 式の意味解析
                let assign = Box::new(Assign::check(ctx, *assign).await?);

                // 型チェック
                let var = VarDeclChecker::find(&mut ctx.var_decl, &ident).await?;
                ty_equals(&var.ty, assign.ty())?;

                Ok(Assign::ShiftRa { span, var, assign })
            }
            ast::Assign::LogicOr { or } => {
                Ok(Assign::LogicOr {
                    or: LogicOr::check(ctx, or).await?,
                })
            }
        }
    }
}

impl Typed for Assign<'_> {
    fn ty(&self) -> &Arc<Type> {
        match self {
            Assign::Normal { var, .. } => &var.ty,
            Assign::Plus { var, .. } => &var.ty,
            Assign::Minus { var, .. } => &var.ty,
            Assign::ShiftL { var, .. } => &var.ty,
            Assign::ShiftR { var, .. } => &var.ty,
            Assign::ShiftRa { var, .. } => &var.ty,
            Assign::LogicOr { or } => or.ty(),
        }
    }
}
