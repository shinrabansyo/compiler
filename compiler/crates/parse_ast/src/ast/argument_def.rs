use super::Visitor;

#[derive(Debug)]
pub struct ArgumentDef {
    pub ident: String,
    pub ty: String,
}

impl From<Visitor<'_>> for ArgumentDef {
    fn from(mut visitor: Visitor<'_>) -> Self {
        ArgumentDef {
            ident: visitor.expect_leaf().1.to_string(),
            ty: visitor.expect_leaf().1.to_string(),
        }
    }
}
