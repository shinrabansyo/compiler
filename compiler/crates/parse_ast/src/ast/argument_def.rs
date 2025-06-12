use super::Visitor;

#[derive(Debug)]
pub struct ArgumentDef {
    pub namespace: String,
    pub ident: String,
    pub ty: String,
}

impl From<(String, Visitor<'_>)> for ArgumentDef {
    fn from((namespace, mut visitor): (String, Visitor<'_>)) -> Self {
        ArgumentDef {
            namespace,
            ident: visitor.expect_leaf().1.to_string(),
            ty: visitor.expect_leaf().1.to_string(),
        }
    }
}
