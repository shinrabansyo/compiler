use super::{Value, Visitor};

#[derive(Debug)]
pub struct Call {
    pub ident: String,
    pub args: Vec<Value>,
}

impl From<Visitor<'_>> for Call {
    fn from(mut visitor: Visitor<'_>) -> Self {
        Call {
            ident: visitor.expect_leaf().1.to_string(),
            args: visitor.expect_nodes::<Value>(),
        }
    }
}
