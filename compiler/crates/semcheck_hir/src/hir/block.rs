use sb_compiler_parse_ast as ast;

use super::{Stmt, SemCheckFrom, InDep};

#[derive(Debug)]
pub struct Block<'input> {
    pub stmts: Vec<Stmt<'input>>,
}

impl<'input> SemCheckFrom<InDep, ast::Block<'input>> for Block<'input> {
    async fn check0(mut ctx: InDep, block: ast::Block<'input>) -> anyhow::Result<Self>
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
