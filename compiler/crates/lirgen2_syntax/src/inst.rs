mod add;    pub use add::Add;
mod li;     pub use li::Li;

pub trait Inst {
    fn process(&self);
}
