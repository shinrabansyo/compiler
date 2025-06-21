use sb_compiler_parse_ast as ast;
use sb_compiler_semcheck_impl_vardecl::{VarId, VarDeclChecker};

use super::{SemCheckFrom, Dep};

#[derive(Debug)]
pub enum InlineAsmReg {
    RawReg {
        num: u8,
    },
    TmpReg {
        num: u32,
    },
    Var {
        id: VarId
    },
}

impl<'input> SemCheckFrom<Dep<'_>, ast::InlineAsmReg<'input>> for InlineAsmReg {
    async fn check0(ctx: Dep<'_>, reg: ast::InlineAsmReg<'input>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match reg {
            ast::InlineAsmReg::RawReg { num } => {
                Ok(InlineAsmReg::RawReg { num })
            }
            ast::InlineAsmReg::TmpReg { num } => {
                Ok(InlineAsmReg::TmpReg { num })
            }
            ast::InlineAsmReg::Var { name } => {
                let var_id = VarDeclChecker::find(
                    &mut ctx.var_decl,
                    name.as_str(),
                ).await?;

                Ok(InlineAsmReg::Var { id: var_id })
            }
        }
    }
}
