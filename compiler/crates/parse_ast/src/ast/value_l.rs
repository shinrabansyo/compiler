use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_parse_syntax::{SBToken, SBRule};

use super::{StructAccess, Visitor};

#[derive(Debug)]
pub enum ValueL<'src> {
    Var {
        span: Span<'src>,
    },
    StructAccess {
        struct_access: StructAccess<'src>,
    },
}

impl<'src> From<Visitor<'src>> for ValueL<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        match visitor.peek() {
            // 変数
            (Some(SBToken::Ident), None) => {
                ValueL::Var {
                    span: visitor.span(),
                }
            }
            // 構造体アクセス
            (None, Some(SBRule::StructAccess)) => {
                ValueL::StructAccess {
                    struct_access: visitor.expect_node::<StructAccess>(),
                }
            }
            _ => unreachable!(),
        }
    }
}

impl<'src> Spanned<'src> for ValueL<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            ValueL::Var { span } => *span,
            ValueL::StructAccess { struct_access } => struct_access.span(),
        }
    }
}
