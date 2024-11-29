use std::fmt::{Debug, Display};

pub struct FLoad;

impl Debug for FLoad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Display for FLoad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // レジスタの復元
        writeln!(f, "lw r1 = r3[-4]")?;
        writeln!(f, "lw r6 = r3[-8]")?;
        writeln!(f, "lw r20 = r3[-12]")?;
        writeln!(f, "lw r21 = r3[-16]")?;
        writeln!(f, "lw r22 = r3[-20]")?;
        writeln!(f, "lw r23 = r3[-24]")?;
        writeln!(f, "lw r24 = r3[-28]")?;
        writeln!(f, "lw r25 = r3[-32]")?;
        writeln!(f, "lw r26 = r3[-36]")?;
        writeln!(f, "lw r27 = r3[-40]")?;
        writeln!(f, "lw r28 = r3[-44]")?;
        writeln!(f, "lw r29 = r3[-48]")?;
        writeln!(f, "addi r2 = r2, 48")?;

        // フレームポインタの復元
        writeln!(f, "lw r3 = r2[0]")?;
        write!(f, "addi r2 = r2, 4")
    }
}

impl FLoad {
    pub fn new() -> Self {
        FLoad
    }
}
