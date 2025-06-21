use sb_compiler_parse_ast as ast;
use sb_compiler_semcheck_impl_vardecl::{VarDeclChecker, VarId};

use super::{LogicOr, SemCheckFrom, Dep};

#[derive(Debug)]
pub enum Assign<'input> {
    Normal {
        ident: VarId,
        assign: Box<Assign<'input>>,
    },
    Plus {
        ident: VarId,
        assign: Box<Assign<'input>>,
    },
    Minus {
        ident: VarId,
        assign: Box<Assign<'input>>,
    },
    ShiftL {
        ident: VarId,
        assign: Box<Assign<'input>>,
    },
    ShiftR {
        ident: VarId,
        assign: Box<Assign<'input>>,
    },
    ShiftRa {
        ident: VarId,
        assign: Box<Assign<'input>>,
    },
    LogicOr {
        or: LogicOr<'input>,
    }
}

impl<'input> SemCheckFrom<Dep<'_>, ast::Assign<'input>> for Assign<'input> {
    async fn check0(ctx: Dep<'_>, assign: ast::Assign<'input>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match assign {
            ast::Assign::Normal { ident, assign } => {
                let var_id = VarDeclChecker::find(
                    &mut ctx.var_decl,
                    ident.as_str(),
                ).await?;

                Ok(Assign::Normal {
                    ident: var_id,
                    assign: Box::new(Assign::check(ctx, *assign).await?),
                })
            }
            ast::Assign::Plus { ident, assign } => {
                let var_id = VarDeclChecker::find(
                    &mut ctx.var_decl,
                    ident.as_str(),
                ).await?;

                Ok(Assign::Plus {
                    ident: var_id,
                    assign: Box::new(Assign::check(ctx, *assign).await?),
                })
            }
            ast::Assign::Minus { ident, assign } => {
                let var_id = VarDeclChecker::find(
                    &mut ctx.var_decl,
                    ident.as_str(),
                ).await?;

                Ok(Assign::Minus {
                    ident: var_id,
                    assign: Box::new(Assign::check(ctx, *assign).await?),
                })
            }
            ast::Assign::ShiftL { ident, assign } => {
                let var_id = VarDeclChecker::find(
                    &mut ctx.var_decl,
                    ident.as_str(),
                ).await?;

                Ok(Assign::ShiftL {
                    ident: var_id,
                    assign: Box::new(Assign::check(ctx, *assign).await?),
                })
            }
            ast::Assign::ShiftR { ident, assign } => {
                let var_id = VarDeclChecker::find(
                    &mut ctx.var_decl,
                    ident.as_str(),
                ).await?;

                Ok(Assign::ShiftR {
                    ident: var_id,
                    assign: Box::new(Assign::check(ctx, *assign).await?),
                })
            }
            ast::Assign::ShiftRa { ident, assign } => {
                let var_id = VarDeclChecker::find(
                    &mut ctx.var_decl,
                    ident.as_str(),
                ).await?;

                Ok(Assign::ShiftRa {
                    ident: var_id,
                    assign: Box::new(Assign::check(ctx, *assign).await?),
                })
            }
            ast::Assign::LogicOr { or } => {
                Ok(Assign::LogicOr {
                    or: LogicOr::check(ctx, or).await?,
                })
            }
        }
    }
}
