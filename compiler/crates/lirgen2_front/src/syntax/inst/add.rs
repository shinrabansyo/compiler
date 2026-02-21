use crate::translate::{Translatable, TranslateContext};
use crate::var::LirVar;

#[derive(Debug)]
pub struct Add {
    pub src1: LirVar,
    pub src2: LirVar,
}

impl Translatable for Add {
    fn translate(&mut self, _: &mut TranslateContext) {
        println!("Add {:?}, {:?}", self.src1, self.src2);
    }
}

impl Add {
    pub fn apply(src1: LirVar, src2: LirVar) -> Self {
        Add { src1, src2 }
    }
}
