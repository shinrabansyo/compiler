use sb_compiler_parse_ast as ast;
use sb_compiler_semcheck_impl_vardecl::{VarId, VarDeclChecker};

use super::{SemCheckFrom, Dep};

#[derive(Debug)]
pub enum InlineAsmOperand {
    Reg {
        num: u8,
    },
    Var {
        id: VarId
    },
}

impl<'input> SemCheckFrom<Dep<'_>, ast::InlineAsmOperandL<'input>> for InlineAsmOperand {
    async fn check0(ctx: Dep<'_>, operand: ast::InlineAsmOperandL<'input>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match operand {
            ast::InlineAsmOperandL::Reg { num } => {
                Ok(InlineAsmOperand::Reg { num })
            }
            ast::InlineAsmOperandL::Var { name } => {
                let var_name = name.as_str();
                let var_id = match VarDeclChecker::exists(&ctx.var_decl, var_name) {
                    Some(var_id) => var_id,
                    None => VarDeclChecker::register(&mut ctx.var_decl, var_name).unwrap(),
                };

                Ok(InlineAsmOperand::Var { id: var_id })
            }
        }
    }
}

impl<'input> SemCheckFrom<Dep<'_>, ast::InlineAsmOperandR<'input>> for InlineAsmOperand {
    async fn check0(ctx: Dep<'_>, operand: ast::InlineAsmOperandR<'input>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match operand {
            ast::InlineAsmOperandR::Reg { num } => {
                Ok(InlineAsmOperand::Reg { num })
            }
            ast::InlineAsmOperandR::Var { name } => {
                let var_id = VarDeclChecker::find(
                    &ctx.var_decl,
                    name.as_str(),
                ).await?;

                Ok(InlineAsmOperand::Var { id: var_id })
            }
        }
    }
}
