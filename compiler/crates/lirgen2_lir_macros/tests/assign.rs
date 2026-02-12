use sb_compiler_lirgen2_lir::inst::{Add, Li};
use sb_compiler_lirgen2_lir::var::LirVarIssuer;
use sb_compiler_lirgen2_lir::prelude::*;
use sb_compiler_lirgen2_lir_macros::lir;

#[test]
fn assign_1() {
    let result = lir! {
        let a = Li::<0>::new();
        let b = Li::<1>::new();
        Add::new(a, b);
    };
    result.process(&mut LirVarIssuer::new());
}
