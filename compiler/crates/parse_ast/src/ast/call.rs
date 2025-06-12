use super::{Value, Visitor};

#[derive(Debug)]
pub struct Call {
    pub namespace: String,
    pub ident: String,
    pub args: Vec<Value>,
}

impl From<(String, Visitor<'_>)> for Call {
    fn from((namespace, mut visitor): (String, Visitor<'_>)) -> Self {
        Call {
            namespace,
            ident: visitor.expect_leaf().1.to_string(),
            args: visitor.expect_nodes::<Value>(),
        }
    }
}
