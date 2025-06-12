use super::{ArgumentDef, Block, Visitor};

#[derive(Debug)]
pub struct FuncDef<'input> {
    pub ident: &'input str,
    pub args: Vec<ArgumentDef>,
    pub ret_ty: Option<&'input str>,
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
