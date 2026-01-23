use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::Spanned;
use sb_compiler_semcheck_impl_type::{Typed, Type};

use super::{FuncDef, StructDef, SemCheck, InDep};

#[derive(Debug)]
pub enum Top<'src> {
    StructDef {
        struct_def: StructDef<'src>,
    },
    FuncDef {
        func_def: FuncDef<'src>,
    },
}

impl<'src> SemCheck<InDep<'src>, ast::Top<'src>> for Top<'src> {
    async fn check0(ctx: InDep<'src>, top: ast::Top<'src>) -> miette::Result<Top<'src>>
    where
        Self: Sized,
    {
        match top {
            ast::Top::StructDef { struct_def } => {
                Ok(Top::StructDef {
                    struct_def: StructDef::check(ctx, struct_def).await?,
                })
            },
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
            Top::StructDef { struct_def } => struct_def.span(),
            Top::FuncDef { func_def } => func_def.span(),
        }
    }
}

impl Typed for Top<'_> {
    fn ty(&self) -> Arc<Type> {
        match self {
            Top::StructDef { struct_def } => struct_def.ty(),
            Top::FuncDef { func_def } => func_def.ty(),
        }
    }
}
