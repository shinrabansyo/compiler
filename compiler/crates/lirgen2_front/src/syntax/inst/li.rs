use crate::translate::{Translatable, TranslateContext};

#[derive(Debug)]
pub struct Li<const IMM: i32>;

impl<const IMM: i32> Translatable for Li<IMM> {
    fn translate(&mut self, _: &mut TranslateContext) {
        println!("Li {}", IMM);
    }
}
