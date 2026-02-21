use std::fmt::Write;
use std::fmt;

use sb_compiler_lirgen2_front::syntax::Terminal;

use crate::{LirDebug, LirDebugCtx};

impl<F: Write> LirDebug<F> for Terminal {
    fn fmt(&self, _: &mut F, _: &mut LirDebugCtx) -> fmt::Result {
        Ok(())
    }
}
