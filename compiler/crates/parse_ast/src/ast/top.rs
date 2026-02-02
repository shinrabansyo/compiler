use sb_compiler_parse_cst::Spanned;
use sb_compiler_parse_syntax::SBRule;

use super::{FuncDef, StructDef, Visitor};

#[derive(Debug)]
pub enum Top<'src> {
    StructDef {
        struct_def: StructDef<'src>,
    },
    FuncDef {
        func_def: FuncDef<'src>,
    },
}

impl<'src> From<Visitor<'src>> for Top<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        match visitor.peek().1 {
            Some(SBRule::StructDef) => Top::StructDef {
                struct_def: visitor.expect_node::<StructDef>(),
            },
            Some(SBRule::FuncDef) => Top::FuncDef {
                func_def: visitor.expect_node::<FuncDef>(),
            },
            _ => unreachable!(),
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
