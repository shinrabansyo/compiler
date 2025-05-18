// mod ir;

// pub use ir::*;

pub use LirInst::*;

#[macro_export]
macro_rules! lir {
    // ラベル配置
    (LLabel $label:ident) => {
        LirBlock::LLabel { label: $label }
    };

    (GLabel $label:expr) => {
        LirBlock::GLabel { label: $label }
    };

    // 0-レジスタ && 命令引数なし
    ($inst:ident) => {
        LirBlock::Inst {
            inst: $inst,
            dst: 0,
            src1: 0,
            src2: 0,
        }
    };

    // 0-レジスタ && 命令引数あり
    ($inst:ident ( $($arg:expr),* )) => {
        LirBlock::Inst {
            inst: $inst ( $($arg),* ),
            dst: 0,
            src1: 0,
            src2: 0,
        }
    };

    // 1-レジスタ && 命令引数あり
    ($inst:ident ( $($arg:expr),* ) $dst:expr) => {
        LirBlock::Inst {
            inst: $inst ( $($arg),* ),
            dst: $dst,
            src1: 0,
            src2: 0,
        }
    };

    // 1-レジスタ && 命令引数なし
    ($inst:ident $dst:expr) => {
        LirBlock::Inst {
            inst: $inst,
            dst: $dst,
            src1: 0,
            src2: 0,
        }
    };

    // 2-レジスタ && 命令引数あり
    ($inst:ident ( $($arg:expr),* ) $dst:expr, $src:expr) => {
        LirBlock::Inst {
            inst: $inst ( $($arg),* ),
            dst: $dst,
            src1: $src,
            src2: 0,
        }
    };

    // 2-レジスタ && 命令引数なし
    ($inst:ident $dst:expr, $src:expr) => {
        LirBlock::Inst {
            inst: $inst,
            dst: $dst,
            src1: $src,
            src2: 0,
        }
    };

    // 3-レジスタ && 命令引数あり
    ($inst:ident ( $($arg:expr),* ) $dst:expr, $src1:expr, $src2:expr) => {
        LirBlock::Inst {
            inst: $inst ( $($arg),* ),
            dst: $dst,
            src1: $src1,
            src2: $src2,
        }
    };

    // 3-レジスタ && 命令引数なし
    ($inst:ident $dst:expr, $src1:expr, $src2:expr) => {
        LirBlock::Inst {
            inst: $inst,
            dst: $dst,
            src1: $src1,
            src2: $src2,
        }
    };
}

#[derive(Debug)]
pub enum LirInst {
    // Nop
    Nop,

    // 整数演算 (imm 使用)
    Li(i32),
    Addi(i32),
    Subi(i32),
    Andi(i32),
    Ori(i32),
    Xori(i32),
    ShiftLi(i32),
    ShiftRi(i32),
    ShiftRai(i32),

    // 整数演算 (imm 不使用)
    Add,
    Sub,
    And,
    Or,
    Xor,
    ShiftL,
    ShiftR,
    ShiftRa,

    // 分岐
    Beq(i32),
    Bne(i32),
    Blt(i32),
    Ble(i32),
    Jmp(i32),
    JmpLabel(u32),
    Call(String),

    // 関数
    FnPrologue,
    FnEpilogue(String),
    FnReturn(String),
}

#[derive(Debug)]
pub enum LirBlock {
    // 単一で実行されるブロック (func, if, ...)
    Single {
        result_reg: u32,
        lirs: Vec<LirBlock>,
    },

    // 多重で実行されるブロック (for, while, ...)
    Multiple {
        lirs: Vec<LirBlock>,
    },

    // ラベル
    LLabel {
        label: u32,
    },
    GLabel {
        label: String,
    },

    // 単一命令
    Inst {
        inst: LirInst,
        dst: u32,
        src1: u32,
        src2: u32,
    },
}

impl LirBlock {
    pub fn result_reg(&self) -> u32 {
        match self {
            LirBlock::Single { result_reg, .. } => *result_reg,
            LirBlock::Multiple { .. } => 0,
            _ => panic!("LirBlock::result_reg() called on non-node"),
        }
    }
}

#[derive(Debug)]
pub enum LirTopElem {
    Function {
        namespace: String,
        name: String,
        body: LirBlock,
    }
}
