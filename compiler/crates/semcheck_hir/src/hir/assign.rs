use sb_compiler_parse_ast as ast;
use sb_compiler_semcheck_impl_vardecl::{Var, VarDeclChecker};

use super::{LogicOr, SemCheckFrom, Dep};

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

impl<'src> SemCheckFrom<Dep<'_, 'src>, ast::Assign<'src>> for Assign<'src> {
    async fn check0(ctx: Dep<'_, 'src>, assign: ast::Assign<'src>) -> anyhow::Result<Self>
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
