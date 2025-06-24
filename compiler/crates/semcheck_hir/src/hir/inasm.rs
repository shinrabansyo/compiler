use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_type::r#type::{Type, Void};
use sb_compiler_type::Typed;

use super::{InlineAsmInst, SemCheck, InDep};

#[derive(Debug)]
pub struct InlineAsm<'src> {
    pub span: Span<'src>,
    pub insts: Vec<InlineAsmInst<'src>>,
}

impl<'src> SemCheck<InDep<'src>, ast::InlineAsm<'src>> for InlineAsm<'src> {
    async fn check0(mut ctx: InDep<'src>, inasm: ast::InlineAsm<'src>) -> miette::Result<Self>
    where
        Self: Sized,
    {
        // 命令を順に意味解析
        let mut insts = vec![];
        for inst in inasm.insts {
            insts.push(InlineAsmInst::check(&mut ctx, inst).await?);
        }

        Ok(InlineAsm { span: inasm.span, insts })
    }
}

impl<'src> Spanned<'src> for InlineAsm<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}

impl Typed for InlineAsm<'_> {
    fn ty(&self) -> Arc<Type> {
        Void.ty()
    }
}
