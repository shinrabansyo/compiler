use sb_compiler_parse_ast as ast;
use sb_compiler_semcheck_impl_type_decl::Type;

use super::{FuncDef, SemCheck, InDep};

#[derive(Debug)]
pub enum Top<'src> {
    FuncDef {
        func_def: FuncDef<'src>,
    },
}

impl<'src> SemCheck<InDep<'src>, ast::Top<'src>> for Top<'src> {
    async fn check0(ctx: InDep<'src>, top: ast::Top<'src>) -> anyhow::Result<Top<'src>>
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

    fn ty(&self) -> &Type {
        match self {
            Top::FuncDef { func_def } => func_def.ty(),
        }
    }
}
