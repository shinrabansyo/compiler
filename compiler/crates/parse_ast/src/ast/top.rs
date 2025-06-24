use sb_compiler_parse_cst::Spanned;

use super::{FuncDef, Visitor};

#[derive(Debug)]
pub enum Top<'src> {
    FuncDef {
        func_def: FuncDef<'src>,
    },
}

impl<'src> From<Visitor<'src>> for Top<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        Top::FuncDef {
            func_def: visitor.expect_node::<FuncDef>(),
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
