use std::fmt::Write;
use std::fmt;

use sb_compiler_lirgen2_front::syntax::{LirSyntax, Assign};

use crate::{LirDebug, LirDebugCtx};

impl<F, S> LirDebug<F> for Assign<S>
where
    F: Write,
    S: LirSyntax + LirDebug<F>,
{
    fn fmt(&self, f: &mut F, ctx: &mut LirDebugCtx) -> fmt::Result {
        self.dst.set(ctx.issued_var);
        ctx.issued_var += 1;

        write!(f, "let {:?} = ", self.dst)?;
        self.rhs.fmt(f, ctx)?;
        writeln!(f)
    }
}
