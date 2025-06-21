use sb_compiler_parse_ast as ast;

use super::{InlineAsmInst, SemCheckFrom, InDep};

#[derive(Debug)]
pub struct InlineAsm<'input> {
    pub insts: Vec<InlineAsmInst<'input>>,
}

impl<'input> SemCheckFrom<InDep<'input>, ast::InlineAsm<'input>> for InlineAsm<'input> {
    async fn check0(mut ctx: InDep<'input>, inasm: ast::InlineAsm<'input>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let mut insts = vec![];
        for inst in inasm.insts {
            insts.push(InlineAsmInst::check(&mut ctx, inst).await?);
        }

        Ok(InlineAsm { insts })
    }
}
