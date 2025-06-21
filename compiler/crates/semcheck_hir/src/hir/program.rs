use sb_compiler_parse_ast as ast;
use sb_compiler_semcheck_async::prelude::*;

use super::{Top, SemCheckFrom, InDep};

#[derive(Debug)]
pub struct Program<'input> {
    pub top_elems: Vec<Top<'input>>,
}

impl<'input> SemCheckFrom<InDep, ast::Program<'input>> for Program<'input> {
    async fn check0(ctx: InDep, program: ast::Program<'input>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let top_elems = program
            .top_elems
            .into_iter()
            .map(|top| Top::check(ctx.clone(), top))
            .join_all()
            .await
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(Program { top_elems })
    }
}
