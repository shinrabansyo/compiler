use sb_compiler_parse_ast as ast;

use super::{FuncDef, SemCheckFrom, InDep};

#[derive(Debug)]
pub enum Top<'input> {
    FuncDef {
        func_def: FuncDef<'input>,
    },
}

impl<'input> SemCheckFrom<InDep, ast::Top<'input>> for Top<'input> {
    async fn check0(ctx: InDep, top: ast::Top<'input>) -> anyhow::Result<Top<'input>>
    where
        Self: Sized,
    {
        match top {
            ast::Top::FuncDef { func_def } => {
                Ok(Top::FuncDef {
                    func_def: FuncDef::check(ctx, func_def).await?,
                })
            }
        }
    }
}
