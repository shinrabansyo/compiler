use sb_compiler_lirgen2_syntax::inst::{Add, Li};
use sb_compiler_lirgen2_syntax::var::LirVarIssuer;
use sb_compiler_lirgen2_syntax::prelude::*;
use sb_compiler_lirgen2_syntax_macros::lir;

#[test]
fn assign_1() {
    let result = lir! {
        let a = Li::<0>::new();
        let b = Li::<1>::new();
        Add::new(a, b);
    };
    result.process(&mut LirVarIssuer::new());
}
