mod li;         pub use li::Li;
mod add;        pub use add::Add;
mod sub;        pub use sub::Sub;
mod beq;        pub use beq::Beq;
mod bne;        pub use bne::Bne;
mod blt;        pub use blt::Blt;
mod ble;        pub use ble::Ble;
mod call;       pub use call::Call;
mod push;       pub use push::Push;
mod pop;        pub use pop::Pop;
mod sw;         pub use sw::Sw;
mod lw;         pub use lw::Lw;
mod label;      pub use label::Label;
mod fsave;      pub use fsave::FSave;
mod fload;      pub use fload::FLoad;
mod varalloc;   pub use varalloc::VarAlloc;
mod varfree;    pub use varfree::VarFree;
mod r#return;   pub use r#return::Return;

use std::fmt::Display;

#[macro_export]
macro_rules! lir {
    ($name:ident $($arg:expr),* ) => {
        LIR::$name($name::new($($arg),*))
    };
}

#[derive(Debug)]
pub enum LIR {
    // 整数演算
    Li(Li),
    Add(Add),
    Sub(Sub),

    // 分岐
    Beq(Beq),
    Bne(Bne),
    Blt(Blt),
    Ble(Ble),
    Call(Call),

    // スタック操作
    Push(Push),
    Pop(Pop),

    // メモリ操作
    Sw(Sw),
    Lw(Lw),

    // 関数
    Label(Label),
    FSave(FSave),
    FLoad(FLoad),
    VarAlloc(VarAlloc),
    VarFree(VarFree),
    Return(Return),
}

impl Display for LIR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LIR::Li(li) => write!(f, "{}", li),
            LIR::Add(add) => write!(f, "{}", add),
            LIR::Sub(sub) => write!(f, "{}", sub),
            LIR::Beq(beq) => write!(f, "{}", beq),
            LIR::Bne(bne) => write!(f, "{}", bne),
            LIR::Blt(blt) => write!(f, "{}", blt),
            LIR::Ble(ble) => write!(f, "{}", ble),
            LIR::Call(jmp) => write!(f, "{}", jmp),
            LIR::Push(push) => write!(f, "{}", push),
            LIR::Pop(pop) => write!(f, "{}", pop),
            LIR::Sw(sw) => write!(f, "{}", sw),
            LIR::Lw(lw) => write!(f, "{}", lw),
            LIR::Label(label) => write!(f, "{}", label),
            LIR::FSave(fsave) => write!(f, "{}", fsave),
            LIR::FLoad(fload) => write!(f, "{}", fload),
            LIR::VarAlloc(varalloc) => write!(f, "{}", varalloc),
            LIR::VarFree(varfree) => write!(f, "{}", varfree),
            LIR::Return(r#return) => write!(f, "{}", r#return),
        }
    }
}
