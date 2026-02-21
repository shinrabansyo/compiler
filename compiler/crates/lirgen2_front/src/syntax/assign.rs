use crate::translate::{Translatable, TranslateContext};
use crate::var::LirVar;
use super::LirSyntax;

#[derive(Debug, PartialEq, Eq)]
pub struct Assign<S: LirSyntax> {
    dst: LirVar,
    rhs: S,
}

impl<S: LirSyntax> Translatable for Assign<S> {
    fn translate(&mut self, ctx: &mut TranslateContext) {
        ctx.issue_var(&mut self.dst);
        print!("let {:?} = ", self.dst);
        self.rhs.translate(ctx);
    }
}

impl<S: LirSyntax> Assign<S> {
    pub fn new(rhs: S) -> (LirVar, Self) {
        let dst = LirVar::new();
        let assign = Assign { dst: dst.clone(), rhs };
        (dst, assign)
    }
}
