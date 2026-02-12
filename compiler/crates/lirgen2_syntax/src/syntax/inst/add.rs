use crate::var::{LirVar, LirVarIssuer};
use crate::syntax::LirSyntax;

#[derive(Debug)]
pub struct Add {
    pub src1: LirVar,
    pub src2: LirVar,
}

impl LirSyntax for Add {
    fn process(&self, _: &mut LirVarIssuer) {
        println!("Add {:?}, {:?}", self.src1, self.src2);
    }
}

impl Add {
    pub fn apply(src1: LirVar, src2: LirVar) -> Self {
        Add { src1, src2 }
    }
}
