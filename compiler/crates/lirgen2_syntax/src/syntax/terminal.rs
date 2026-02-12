use crate::var::LirVarIssuer;
use super::LirSyntax;

#[derive(Debug, PartialEq, Eq)]
pub struct Terminal;

impl LirSyntax for Terminal {
    fn process(&self, _: &mut LirVarIssuer) { }
}
