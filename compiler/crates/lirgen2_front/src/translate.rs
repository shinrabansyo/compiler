use crate::var::LirVar;

pub trait Translatable {
    fn translate(&mut self, ctx: &mut TranslateContext);
}

pub struct TranslateContext {
    issued_var_count: u32,
}

impl TranslateContext {
    pub fn new() -> Self {
        TranslateContext {
            issued_var_count: 0,
        }
    }

    pub fn issue_var(&mut self, var: &mut LirVar) {
        var.set(self.issued_var_count);
        self.issued_var_count += 1;
    }
}
