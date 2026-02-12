use crate::var::LirVar;
use super::Inst;

#[derive(Debug)]
pub struct Add {
    pub src1: LirVar,
    pub src2: LirVar,
}

impl Inst for Add {
    fn process(&self) {
        println!("Add {:?}, {:?}", self.src1, self.src2);
    }
}

impl Add {
    pub fn new(src1: LirVar, src2: LirVar) -> Self {
        Add { src1, src2 }
    }
}
