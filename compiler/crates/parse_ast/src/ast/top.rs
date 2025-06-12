use super::{FuncDef, Visitor};

#[derive(Debug)]
pub enum Top {
    FuncDef {
        namespace: String,
        func_def: FuncDef,
    },
}

impl From<(String, Visitor<'_>)> for Top {
    fn from((namespace, mut visitor): (String, Visitor<'_>)) -> Self {
        Top::FuncDef {
            namespace,
            func_def: visitor.expect_node::<FuncDef>(),
        }
    }
}
