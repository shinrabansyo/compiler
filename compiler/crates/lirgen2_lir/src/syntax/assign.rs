use crate::inst::Inst;
use crate::var::{LirVar, LirVarIssuer};
use super::LirSyntax;

#[derive(Debug, PartialEq, Eq)]
pub struct Assign<I: Inst> {
    dst: LirVar,
    inst: I,
}

impl<I: Inst> LirSyntax for Assign<I> {
    fn process(&self, ctx: &mut LirVarIssuer) {
        ctx.issue(&self.dst);
        print!("let {:?} = ", self.dst);
        self.inst.process();
    }
}

impl<I: Inst> Assign<I> {
    pub fn new(inst: I) -> (LirVar, Self) {
        let dst = LirVar::new();
        let assign = Assign { dst: dst.clone(), inst };
        (dst, assign)
    }
}
