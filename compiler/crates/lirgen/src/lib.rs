mod gen;

use sb_compiler_lirgen_ir::LirTopElem;
use sb_compiler_semcheck_hir::Program;

use gen::{lirgen_program, GenContext};

pub fn lirgen<'src, I>(hirs: I) -> Vec<LirTopElem>
where
    I: Iterator<Item = Program<'src>>,
{
    let lirgen = |hir| {
        let mut ctx = GenContext::default();
        lirgen_program(&mut ctx, &hir)
    };

    hirs.flat_map(lirgen)
        .collect::<Vec<_>>()
}
