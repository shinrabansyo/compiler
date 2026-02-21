use crate::translate::{Translatable, TranslateContext};

#[derive(Debug)]
pub struct Terminal;

impl Translatable for Terminal {
    fn translate(&mut self, _: &mut TranslateContext) { }
}
