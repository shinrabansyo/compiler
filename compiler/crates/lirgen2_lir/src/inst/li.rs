use super::Inst;

#[derive(Debug)]
pub struct Li<const IMM: i32>;

impl<const IMM: i32> Inst for Li<IMM> {
    fn process(&self) {
        println!("Li {}", IMM);
    }
}

impl<const IMM: i32> Li<IMM> {
    pub fn new() -> Self {
        Li
    }
}
