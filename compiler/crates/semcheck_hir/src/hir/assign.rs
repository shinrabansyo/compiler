use sb_compiler_parse_ast as ast;
use sb_compiler_semcheck_impl_vardecl::{Var, VarDeclChecker};

use super::{LogicOr, SemCheckFrom, Dep};

#[derive(Debug)]
pub enum Assign<'input> {
    Normal {
        var: Var<'input>,
        assign: Box<Assign<'input>>,
    },
    Plus {
        var: Var<'input>,
        assign: Box<Assign<'input>>,
    },
    Minus {
        var: Var<'input>,
        assign: Box<Assign<'input>>,
    },
    ShiftL {
        var: Var<'input>,
        assign: Box<Assign<'input>>,
    },
    ShiftR {
        var: Var<'input>,
        assign: Box<Assign<'input>>,
    },
    ShiftRa {
        var: Var<'input>,
        assign: Box<Assign<'input>>,
    },
    LogicOr {
        or: LogicOr<'input>,
    }
}

impl<'input> SemCheckFrom<Dep<'_, 'input>, ast::Assign<'input>> for Assign<'input> {
    async fn check0(ctx: Dep<'_, 'input>, assign: ast::Assign<'input>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match assign {
            ast::Assign::Normal { ident, assign } => {
                Ok(Assign::Normal {
                    var: VarDeclChecker::find(&mut ctx.var_decl, &ident).await?,
                    assign: Box::new(Assign::check(ctx, *assign).await?),
                })
            }
            ast::Assign::Plus { ident, assign } => {
                Ok(Assign::Plus {
                    var: VarDeclChecker::find(&mut ctx.var_decl, &ident).await?,
                    assign: Box::new(Assign::check(ctx, *assign).await?),
                })
            }
            ast::Assign::Minus { ident, assign } => {
                Ok(Assign::Minus {
                    var: VarDeclChecker::find(&mut ctx.var_decl, &ident).await?,
                    assign: Box::new(Assign::check(ctx, *assign).await?),
                })
            }
            ast::Assign::ShiftL { ident, assign } => {
                Ok(Assign::ShiftL {
                    var: VarDeclChecker::find(&mut ctx.var_decl, &ident).await?,
                    assign: Box::new(Assign::check(ctx, *assign).await?),
                })
            }
            ast::Assign::ShiftR { ident, assign } => {
                Ok(Assign::ShiftR {
                    var: VarDeclChecker::find(&mut ctx.var_decl, &ident).await?,
                    assign: Box::new(Assign::check(ctx, *assign).await?),
                })
            }
            ast::Assign::ShiftRa { ident, assign } => {
                Ok(Assign::ShiftRa {
                    var: VarDeclChecker::find(&mut ctx.var_decl, &ident).await?,
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
