use sb_compiler_parse_cst::Span;

use super::{ArgumentDef, Block, Visitor};

#[derive(Debug)]
pub struct FuncDef<'src> {
    pub ident: Span<'src>,
    pub args: Vec<ArgumentDef<'src>>,
    pub ret_ty: Option<Span<'src>>,
    pub block: Block<'src>,
}

impl<'src> From<Visitor<'src>> for FuncDef<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        let ident = visitor.expect_leaf().1;
        let args = visitor.expect_nodes::<ArgumentDef>();
        let ret_ty = visitor
            .peek()
            .0
            .and_then(|_| Some(visitor.expect_leaf().1));
        let block = visitor.expect_node::<Block>();

        FuncDef { ident, args, ret_ty, block }
    }
}
