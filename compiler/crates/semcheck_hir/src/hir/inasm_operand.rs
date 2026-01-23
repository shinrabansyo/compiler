use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_semcheck_impl_var::decl::{var_register, var_find};
use sb_compiler_semcheck_impl_var::Var;
use sb_compiler_semcheck_impl_type::op::ty_equals;
use sb_compiler_semcheck_impl_type::{Typed, Type, I32};

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
                // 変数参照 or 宣言
                let var = var_find(&mut ctx.var, &name).await;
                let var = match var {
                    Ok(var) => var,
                    Err(_) => var_register(&mut ctx.var, &name, I32.ty()).await.unwrap(),
                };
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
                let var = var_find(&mut ctx.var, &name).await?;
                ty_equals(&I32, &var)?;

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
