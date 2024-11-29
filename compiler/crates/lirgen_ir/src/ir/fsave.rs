use std::fmt::{Debug, Display};

pub struct FSave;

impl Debug for FSave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Display for FSave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // フレームポインタの退避
        writeln!(f, "subi r2 = r2, 4")?;
        writeln!(f, "sw r2[0] = r3")?;
        writeln!(f, "add r3 = r0, r2")?;

        // レジスタの退避
        writeln!(f, "subi r2 = r2, 44")?;
        writeln!(f, "sw r3[-4] = r1")?;
        writeln!(f, "sw r3[-8] = r20")?;
        writeln!(f, "sw r3[-12] = r21")?;
        writeln!(f, "sw r3[-16] = r22")?;
        writeln!(f, "sw r3[-20] = r23")?;
        writeln!(f, "sw r3[-24] = r24")?;
        writeln!(f, "sw r3[-28] = r25")?;
        writeln!(f, "sw r3[-32] = r26")?;
        writeln!(f, "sw r3[-36] = r27")?;
        writeln!(f, "sw r3[-40] = r28")?;
        write!(f, "sw r3[-44] = r29")
    }
}

impl FSave {
    pub fn new() -> Self {
        FSave
    }
}
