use super::{FuncDef, Visitor};

#[derive(Debug)]
pub enum Top<'src> {
    FuncDef {
        func_def: FuncDef<'src>,
    },
}

impl<'src> From<Visitor<'src>> for Top<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        Top::FuncDef {
            func_def: visitor.expect_node::<FuncDef>(),
        }
    }
}
