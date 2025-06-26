mod gen;

use sb_compiler_lirgen_ir::LirTopElem;
use sb_compiler_semcheck_hir::Program;

use gen::{lirgen_program, GenContext};

type HIRs<'src> = Vec<Program<'src>>;
type LIRs = Vec<LirTopElem>;

pub fn lirgen(hirs: HIRs) -> LIRs {
    let lirgen = |hir| {
        let mut ctx = GenContext::default();
        lirgen_program(&mut ctx, hir)
    };

    hirs.into_iter()
        .flat_map(lirgen)
        .collect()
}
