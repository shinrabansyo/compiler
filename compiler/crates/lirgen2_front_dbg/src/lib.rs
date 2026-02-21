mod syntax;

use std::fmt::Write;
use std::fmt;

use sb_compiler_lirgen2_front::syntax::LirSyntax;
use sb_compiler_lirgen2_front::Lir;

#[derive(Debug, Clone, Default)]
struct LirDebugCtx {
    issued_var: u32,
}

trait LirDebug<F: Write> {
    fn fmt(&self, f: &mut F, ctx: &mut LirDebugCtx) -> fmt::Result;
}

impl<F, T, Next> LirDebug<F> for Lir<T, Next>
where
    F: Write,
    T: LirSyntax + LirDebug<F>,
    Next: LirSyntax + LirDebug<F>,
{
    fn fmt(&self, f: &mut F, ctx: &mut LirDebugCtx) -> fmt::Result {
        self.as_ref().fmt(f, ctx)?;
        self.next().fmt(f, ctx)
    }
}

#[allow(private_bounds)]
pub fn debug<F, T, Next>(f: &mut F, lir: &Lir<T, Next>) -> fmt::Result
where
    F: Write,
    T: LirSyntax + LirDebug<F>,
    Next: LirSyntax + LirDebug<F>,
{
    lir.fmt(f, &mut LirDebugCtx::default())
}
