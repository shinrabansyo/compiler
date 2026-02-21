use sb_compiler_lirgen2_front::syntax::*;
use sb_compiler_lirgen2_front::Lir;
use sb_compiler_lirgen2_front_macros::lir;

#[test]
fn assign_1() {
    let lir = lir! {
        let a = Li::<0>;
        let b = Li::<1>;
        Add::apply(a, b);
    };
    // result.process(&mut LirVarIssuer::new());
}
