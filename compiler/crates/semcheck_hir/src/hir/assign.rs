use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_semcheck_impl_vardecl::{Var, VarDeclChecker};
use sb_compiler_type::op::ty_equals;
use sb_compiler_type::r#type::Type;
use sb_compiler_type::Typed;

use super::{LogicOr, SemCheck, Dep};

#[derive(Debug)]
pub enum Assign<'src> {
    Normal {
        var: Var<'src>,
        assign: Box<Assign<'src>>,
    },
    Plus {
        var: Var<'src>,
        assign: Box<Assign<'src>>,
    },
    Minus {
        var: Var<'src>,
        assign: Box<Assign<'src>>,
    },
    ShiftL {
        var: Var<'src>,
        assign: Box<Assign<'src>>,
    },
    ShiftR {
        var: Var<'src>,
        assign: Box<Assign<'src>>,
    },
    ShiftRa {
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
            ast::Assign::Normal { ident, assign } => {
                // 式の意味解析
                let assign = Box::new(Assign::check(ctx, *assign).await?);

                // 型チェック
                let var = VarDeclChecker::find(&mut ctx.var_decl, &ident).await?;
                ty_equals(&var.ty, assign.ty())?;

                Ok(Assign::Normal { var, assign })
            }
            ast::Assign::Plus { ident, assign } => {
                // 式の意味解析
                let assign = Box::new(Assign::check(ctx, *assign).await?);

                // 型チェック
                let var = VarDeclChecker::find(&mut ctx.var_decl, &ident).await?;
                ty_equals(&var.ty, assign.ty())?;

                Ok(Assign::Plus { var, assign })
            }
            ast::Assign::Minus { ident, assign } => {
                // 式の意味解析
                let assign = Box::new(Assign::check(ctx, *assign).await?);

                // 型チェック
                let var = VarDeclChecker::find(&mut ctx.var_decl, &ident).await?;
                ty_equals(&var.ty, assign.ty())?;

                Ok(Assign::Minus { var, assign })
            }
            ast::Assign::ShiftL { ident, assign } => {
                // 式の意味解析
                let assign = Box::new(Assign::check(ctx, *assign).await?);

                // 型チェック
                let var = VarDeclChecker::find(&mut ctx.var_decl, &ident).await?;
                ty_equals(&var.ty, assign.ty())?;

                Ok(Assign::ShiftL { var, assign })
            }
            ast::Assign::ShiftR { ident, assign } => {
                // 式の意味解析
                let assign = Box::new(Assign::check(ctx, *assign).await?);

                // 型チェック
                let var = VarDeclChecker::find(&mut ctx.var_decl, &ident).await?;
                ty_equals(&var.ty, assign.ty())?;

                Ok(Assign::ShiftR { var, assign })
            }
            ast::Assign::ShiftRa { ident, assign } => {
                // 式の意味解析
                let assign = Box::new(Assign::check(ctx, *assign).await?);

                // 型チェック
                let var = VarDeclChecker::find(&mut ctx.var_decl, &ident).await?;
                ty_equals(&var.ty, assign.ty())?;

                Ok(Assign::ShiftRa { var, assign })
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
