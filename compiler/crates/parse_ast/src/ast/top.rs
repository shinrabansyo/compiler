use super::{FuncDef, Visitor};

#[derive(Debug)]
pub enum Top<'input> {
    FuncDef {
        func_def: FuncDef<'input>,
    },
}

impl<'input> From<Visitor<'input>> for Top<'input> {
    fn from(mut visitor: Visitor<'input>) -> Self {
        Top::FuncDef {
            func_def: visitor.expect_node::<FuncDef>(),
        }
    }
}
