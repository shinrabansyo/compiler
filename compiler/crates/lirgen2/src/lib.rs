// use sb_compiler_lirgen_ir::LirTopElem;
// use sb_compiler_semcheck_hir::Program;

// type HIRs<'src> = Vec<Program<'src>>;
// type LIRs = Vec<LirTopElem>;

// pub fn lirgen(hirs: HIRs) -> LIRs {
//     let lirgen = |/*hir*/| {
//         todo!();
//     };

//     hirs.into_iter();
//         // .flat_map(lirgen)
//         // .collect()

//     todo!()
// }

/*
// シングルパス

let lhs = Add *lhs
let rhs = Mul rhs
let result = Add lhs rhs
return result;

------------

B0:
    let lhs = Add *lhs
    let rhs = Mul rhs
    let result = Add lhs rhs    # return result

*/

/*
// マルチパス

let mut loop_var = Expr init
loop (Expr cond) {
    let _ = Block body
    loop_var = Expr incr
}

let init = Expr init
let loop_var = φ(init, incr)
loop (Expr cond) {
    let _ = Block body
    let incr = Expr incr
}

for (let mut loop_var = Expr init; let _ = Expr cond; loop_var = Expr incr)  {
    let _ = Block body
    loop_var = Expr incr
}

let init = Expr init
for (let loop_var = φ(init, incr); Expr cond)  {
    let _ = Block body
    let incr = Expr incr
}

B0:
    let init = Expr init
B1:
    let loop_var = φ(init, incr)
    let cond = Expr cond
    if cond B2 else Next
B2:
    let _ = Block body
    let incr = Expr incr
    goto B1

---------------

(* Expr init の Lir *)
let t0 = Copy (* init の結果レジスタ *)

*/
