use super::{ArgumentDef, Block, Visitor};

#[derive(Debug)]
pub struct FuncDef {
    pub namespace: String,
    pub ident: String,
    pub args: Vec<ArgumentDef>,
    pub ret_ty: Option<String>,
    pub block: Block,
}

impl From<(String, Visitor<'_>)> for FuncDef {
    fn from((namespace, mut visitor): (String, Visitor<'_>)) -> Self {
        let ident = visitor.expect_leaf().1.to_string();
        let args = visitor.expect_nodes::<ArgumentDef>();
        let ret_ty = visitor
            .peek()
            .0
            .and_then(|_| Some(visitor.expect_leaf().1.to_string()));
        let block = visitor.expect_node::<Block>();

        FuncDef { namespace, ident, args, ret_ty, block }
    }
}
