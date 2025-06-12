use sb_compiler_parse_cst::Span;

use super::{ArgumentDef, Block, Visitor};

#[derive(Debug)]
pub struct FuncDef<'input> {
    pub ident: Span<'input>,
    pub args: Vec<ArgumentDef<'input>>,
    pub ret_ty: Option<Span<'input>>,
    pub block: Block<'input>,
}

impl<'input> From<Visitor<'input>> for FuncDef<'input> {
    fn from(mut visitor: Visitor<'input>) -> Self {
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
