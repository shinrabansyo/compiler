use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_parse_syntax::SBRule;

use super::{ArgumentDef, Block, Type, Visitor};

#[derive(Debug)]
pub struct FuncDef<'src> {
    pub span: Span<'src>,
    pub ident: Span<'src>,
    pub args: Vec<ArgumentDef<'src>>,
    pub ret_ty: Option<Type<'src>>,
    pub block: Block<'src>,
}

impl<'src> From<Visitor<'src>> for FuncDef<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        let span = visitor.span();
        let ident = visitor.expect_leaf().1;
        let args = visitor.expect_nodes::<ArgumentDef>();
        let ret_ty = visitor
            .peek()
            .1
            .filter(|rule| rule == &SBRule::Type)
            .and_then(|_| Some(visitor.expect_node::<Type>()));
        let block = visitor.expect_node::<Block>();

        FuncDef { span, ident, args, ret_ty, block }
    }
}

impl<'src> Spanned<'src> for FuncDef<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}
