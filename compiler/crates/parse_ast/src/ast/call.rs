use super::{Value, Visitor};

#[derive(Debug)]
pub struct Call<'input> {
    pub ident: String,
    pub args: Vec<Value<'input>>,
}

impl<'input> From<Visitor<'input>> for Call<'input> {
    fn from(mut visitor: Visitor<'input>) -> Self {
        Call {
            ident: visitor.expect_leaf().1.to_string(),
            args: visitor.expect_nodes::<Value>(),
        }
    }
}
