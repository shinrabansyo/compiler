use sb_compiler_parse_ast as ast;

use super::{InlineAsmInst, SemCheckFrom, InDep};

#[derive(Debug)]
pub struct InlineAsm {
    pub insts: Vec<InlineAsmInst>,
}

impl<'input> SemCheckFrom<InDep, ast::InlineAsm<'input>> for InlineAsm {
    async fn check0(mut ctx: InDep, inasm: ast::InlineAsm<'input>) -> anyhow::Result<Self>
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
