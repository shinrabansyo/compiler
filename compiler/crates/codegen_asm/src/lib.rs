pub mod inst;

use std::fmt::Display;

use inst::AsmInst;

pub struct Asm {
    pub inst: Vec<AsmInst>,
}

impl From<Vec<AsmInst>> for Asm {
    fn from(inst: Vec<AsmInst>) -> Self {
        Asm { inst }
    }
}

impl Display for Asm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "===")?;
        for inst in &self.inst {
            writeln!(f, "{}", inst)?;
        }
        Ok(())
    }
}
