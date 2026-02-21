use sb_compiler_lirgen2_front::syntax::*;
use sb_compiler_lirgen2_front::Lir;
use sb_compiler_lirgen2_front_macros::lir;
use sb_compiler_lirgen2_front_dbg::debug;

macro_rules! assert_lir_eq {
    ($lir:expr, $expected:expr) => {
        let mut f = String::new();
        debug(&mut f, &$lir).unwrap();
        assert_eq!(f, $expected);
    };
}

#[test]
fn assign_1() {
    let lir = lir! {
        let a = Li::<0>;
        let b = Li::<1>;
        Add::apply(a, b);
    };
    let expected =
r#"let v0 = li<0>
let v1 = li<1>
let v2 = add v0 v1
"#;
    assert_lir_eq!(lir, expected);
}

#[test]
fn assign_2() {
    let lir = lir! {
        let a = Li::<0>;
        let b = Li::<1>;
        let c = Add::apply(a, b);
        let d = Li::<2>;
        let e = Li::<3>;
        let e = Add::apply(d, e);
        Add::apply(c, e);
    };
    let expected =
r#"let v0 = li<0>
let v1 = li<1>
let v2 = add v0 v1
let v3 = li<2>
let v4 = li<3>
let v5 = add v3 v4
let v6 = add v2 v5
"#;
    assert_lir_eq!(lir, expected);
}
