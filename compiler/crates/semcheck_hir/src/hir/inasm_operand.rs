use sb_compiler_parse_ast as ast;
use sb_compiler_semcheck_impl_vardecl::{Var, VarDeclChecker};
use sb_compiler_type::op::ty_equals;
use sb_compiler_type::r#type::{I32, Primitive, Type};
use sb_compiler_type::Typed;

use super::{SemCheck, Dep};

#[derive(Debug)]
pub enum InlineAsmOperand<'src> {
    Reg {
        num: u8,
    },
    Var {
        var: Var<'src>,
    },
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::InlineAsmOperandL<'src>> for InlineAsmOperand<'src> {
    async fn check0(ctx: Dep<'_, 'src>, operand: ast::InlineAsmOperandL<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match operand {
            ast::InlineAsmOperandL::Reg { num } => {
                Ok(InlineAsmOperand::Reg { num })
            }
            ast::InlineAsmOperandL::Var { name } => {
                // 変数宣言
                let var = VarDeclChecker::register(
                    &mut ctx.var_decl,
                    &name,
                    Primitive(I32)
                ).unwrap();

                Ok(InlineAsmOperand::Var { var })
            }
        }
    }
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::InlineAsmOperandR<'src>> for InlineAsmOperand<'src> {
    async fn check0(ctx: Dep<'_, 'src>, operand: ast::InlineAsmOperandR<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match operand {
            ast::InlineAsmOperandR::Reg { num } => {
                Ok(InlineAsmOperand::Reg { num })
            }
            ast::InlineAsmOperandR::Var { name } => {
                // 変数の型チェック
                let var = VarDeclChecker::find(&mut ctx.var_decl, &name).await?;
                ty_equals(&var.ty, &Primitive(I32))?;

                Ok(InlineAsmOperand::Var { var })
            }
        }
    }
}

impl Typed for InlineAsmOperand<'_> {
    fn ty(&self) -> &Type {
        &Primitive(I32)
    }
}
