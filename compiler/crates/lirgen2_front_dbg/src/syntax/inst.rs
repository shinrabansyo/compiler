use std::fmt::Write;
use std::fmt;

use sb_compiler_lirgen2_front::syntax::{Add, Li};

use crate::{LirDebug, LirDebugCtx};

impl<F: Write> LirDebug<F> for Add {
    fn fmt(&self, f: &mut F, _: &mut LirDebugCtx) -> fmt::Result {
        write!(f, "add {:?} {:?}", self.src1, self.src2)
    }
}

impl<const IMM: i32, F: Write> LirDebug<F> for Li<IMM> {
    fn fmt(&self, f: &mut F, _: &mut LirDebugCtx) -> fmt::Result {
        write!(f, "li<{}>", IMM)
    }
}
