use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_semcheck_impl_vardecl::{Var, VarDeclChecker};
use sb_compiler_type::op::ty_equals;
use sb_compiler_type::r#type::{I32, Type};
use sb_compiler_type::Typed;

use super::{SemCheck, Dep};

#[derive(Debug)]
pub enum InlineAsmOperand<'src> {
    Reg {
        span: Span<'src>,
        num: u8,
    },
    Var {
        var: Var<'src>,
    },
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::InlineAsmOperandL<'src>> for InlineAsmOperand<'src> {
    async fn check0(ctx: Dep<'_, 'src>, operand: ast::InlineAsmOperandL<'src>) -> miette::Result<Self>
    where
        Self: Sized,
    {
        match operand {
            ast::InlineAsmOperandL::Reg { span, num } => {
                Ok(InlineAsmOperand::Reg { span, num })
            }
            ast::InlineAsmOperandL::Var { name } => {
                // 変数宣言
                let var = VarDeclChecker::register(
                    &mut ctx.var_decl,
                    &name,
                    I32.ty(),
                ).unwrap();

                Ok(InlineAsmOperand::Var { var })
            }
        }
    }
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::InlineAsmOperandR<'src>> for InlineAsmOperand<'src> {
    async fn check0(ctx: Dep<'_, 'src>, operand: ast::InlineAsmOperandR<'src>) -> miette::Result<Self>
    where
        Self: Sized,
    {
        match operand {
            ast::InlineAsmOperandR::Reg { span, num } => {
                Ok(InlineAsmOperand::Reg { span, num })
            }
            ast::InlineAsmOperandR::Var { name } => {
                // 変数の型チェック
                let var = VarDeclChecker::find(&mut ctx.var_decl, &name).await?;
                ty_equals(&var.ty, &I32.ty())?;

                Ok(InlineAsmOperand::Var { var })
            }
        }
    }
}

impl<'src> Spanned<'src> for InlineAsmOperand<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            InlineAsmOperand::Reg { span, .. } => *span,
            InlineAsmOperand::Var { var } => var.span(),
        }
    }
}

impl Typed for InlineAsmOperand<'_> {
    fn ty(&self) -> Arc<Type> {
        I32.ty()
    }
}
