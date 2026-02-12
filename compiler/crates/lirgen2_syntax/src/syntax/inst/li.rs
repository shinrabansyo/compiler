use crate::var::LirVarIssuer;
use crate::syntax::LirSyntax;

#[derive(Debug)]
pub struct Li<const IMM: i32>;

impl<const IMM: i32> LirSyntax for Li<IMM> {
    fn process(&self, _: &mut LirVarIssuer) {
        println!("Li {}", IMM);
    }
}
