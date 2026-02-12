use crate::var::{LirVar, LirVarIssuer};
use super::LirSyntax;

#[derive(Debug, PartialEq, Eq)]
pub struct Assign<S: LirSyntax> {
    dst: LirVar,
    rhs: S,
}

impl<S: LirSyntax> LirSyntax for Assign<S> {
    fn process(&self, ctx: &mut LirVarIssuer) {
        ctx.issue(&self.dst);
        print!("let {:?} = ", self.dst);
        self.rhs.process(ctx);
    }
}

impl<S: LirSyntax> Assign<S> {
    pub fn new(rhs: S) -> (LirVar, Self) {
        let dst = LirVar::new();
        let assign = Assign { dst: dst.clone(), rhs };
        (dst, assign)
    }
}
