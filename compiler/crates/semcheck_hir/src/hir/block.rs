use sb_compiler_parse_ast as ast;

use super::{Stmt, SemCheck, InDep};

#[derive(Debug)]
pub struct Block<'src> {
    pub stmts: Vec<Stmt<'src>>,
}

impl<'src> SemCheck<InDep<'src>, ast::Block<'src>> for Block<'src> {
    async fn check0(mut ctx: InDep<'src>, block: ast::Block<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let mut stmts = vec![];
        for stmt in block.stmts {
            stmts.push(Stmt::check(&mut ctx, stmt).await?);
        }

        Ok(Block { stmts })
    }
}
