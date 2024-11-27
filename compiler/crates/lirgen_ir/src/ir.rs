mod li;     pub use li::Li;
mod add;    pub use add::Add;
mod sub;    pub use sub::Sub;
mod push;   pub use push::Push;
mod pop;    pub use pop::Pop;
mod sw;     pub use sw::Sw;

#[derive(Debug)]
pub enum LIR {
    // 整数演算
    Li(Li),
    Add(Add),
    Sub(Sub),

    // スタック操作
    Push(Push),
    Pop(Pop),

    // メモリ操作
    Sw(Sw),
}
