use super::{FuncDef, Visitor};

#[derive(Debug)]
pub enum Top {
    FuncDef {
        func_def: FuncDef,
    },
}

impl From<Visitor<'_>> for Top {
    fn from(mut visitor: Visitor<'_>) -> Self {
        Top::FuncDef {
            func_def: visitor.expect_node::<FuncDef>(),
        }
    }
}
