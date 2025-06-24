use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::Spanned;
use sb_compiler_type::r#type::Type;
use sb_compiler_type::Typed;

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
}

impl<'src> Spanned<'src> for Top<'src> {
    fn span(&self) -> sb_compiler_parse_cst::Span<'src> {
        match self {
            Top::FuncDef { func_def } => func_def.span(),
        }
    }
}

impl Typed for Top<'_> {
    fn ty(&self) -> &Arc<Type> {
        match self {
            Top::FuncDef { func_def } => func_def.ty(),
        }
    }
}
