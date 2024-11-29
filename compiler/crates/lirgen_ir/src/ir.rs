mod li;         pub use li::Li;
mod add;        pub use add::Add;
mod sub;        pub use sub::Sub;
mod jmp;        pub use jmp::Jmp;
mod push;       pub use push::Push;
mod pop;        pub use pop::Pop;
mod sw;         pub use sw::Sw;
mod lw;         pub use lw::Lw;
mod label;      pub use label::Label;
mod fsave;      pub use fsave::FSave;
mod fload;      pub use fload::FLoad;
mod r#return;   pub use r#return::Return;

#[derive(Debug)]
pub enum LIR {
    // 整数演算
    Li(Li),
    Add(Add),
    Sub(Sub),

    // 分岐
    Jmp(Jmp),

    // スタック操作
    Push(Push),
    Pop(Pop),

    // メモリ操作
    Sw(Sw),
    Lw(Lw),

    // その他
    Label(Label),
    FSave(FSave),
    FLoad(FLoad),
    Return(Return),
}

#[macro_export]
macro_rules! lir {
    ($name:ident : $($arg:expr),* ) => {
        LIR::$name($name::new($($arg),*))
    };
}
