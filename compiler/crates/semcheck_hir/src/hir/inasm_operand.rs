use sb_compiler_parse_ast as ast;
use sb_compiler_semcheck_impl_vardecl::{Var, VarDeclChecker};

use super::{SemCheckFrom, Dep};

#[derive(Debug)]
pub enum InlineAsmOperand<'input> {
    Reg {
        num: u8,
    },
    Var {
        var: Var<'input>,
    },
}

impl<'input> SemCheckFrom<Dep<'_, 'input>, ast::InlineAsmOperandL<'input>> for InlineAsmOperand<'input> {
    async fn check0(ctx: Dep<'_, 'input>, operand: ast::InlineAsmOperandL<'input>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match operand {
            ast::InlineAsmOperandL::Reg { num } => {
                Ok(InlineAsmOperand::Reg { num })
            }
            ast::InlineAsmOperandL::Var { name } => {
                let var = match VarDeclChecker::exists(&ctx.var_decl, &name) {
                    Some(var) => var,
                    None => VarDeclChecker::register(&mut ctx.var_decl, &name).unwrap(),
                };
                Ok(InlineAsmOperand::Var { var })
            }
        }
    }
}

impl<'input> SemCheckFrom<Dep<'_, 'input>, ast::InlineAsmOperandR<'input>> for InlineAsmOperand<'input> {
    async fn check0(ctx: Dep<'_, 'input>, operand: ast::InlineAsmOperandR<'input>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match operand {
            ast::InlineAsmOperandR::Reg { num } => {
                Ok(InlineAsmOperand::Reg { num })
            }
            ast::InlineAsmOperandR::Var { name } => {
                Ok(InlineAsmOperand::Var {
                    var: VarDeclChecker::find(&mut ctx.var_decl, &name).await?,
                })
            }
        }
    }
}
