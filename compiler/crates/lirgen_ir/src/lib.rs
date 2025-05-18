// mod ir;

// pub use ir::*;

pub use LirInst::*;

#[macro_export]
macro_rules! lir {
    // ラベル配置
    (Label $label:ident) => {
        LirTree::Label { label: $label }
    };

    // 0-レジスタ && 命令引数なし
    ($inst:ident) => {
        LirTree::Inst {
            inst: $inst,
            dst: 0,
            src1: 0,
            src2: 0,
        }
    };

    // 0-レジスタ && 命令引数あり
    ($inst:ident ( $($arg:expr),* )) => {
        LirTree::Inst {
            inst: $inst ( $($arg),* ),
            dst: 0,
            src1: 0,
            src2: 0,
        }
    };

    // 1-レジスタ && 命令引数あり
    ($inst:ident ( $($arg:expr),* ) $dst:expr) => {
        LirTree::Inst {
            inst: $inst ( $($arg),* ),
            dst: $dst,
            src1: 0,
            src2: 0,
        }
    };

    // 1-レジスタ && 命令引数なし
    ($inst:ident $dst:expr) => {
        LirTree::Inst {
            inst: $inst,
            dst: $dst,
            src1: 0,
            src2: 0,
        }
    };

    // 2-レジスタ && 命令引数あり
    ($inst:ident ( $($arg:expr),* ) $dst:expr, $src:expr) => {
        LirTree::Inst {
            inst: $inst ( $($arg),* ),
            dst: $dst,
            src1: $src,
            src2: 0,
        }
    };

    // 2-レジスタ && 命令引数なし
    ($inst:ident $dst:expr, $src:expr) => {
        LirTree::Inst {
            inst: $inst,
            dst: $dst,
            src1: $src,
            src2: 0,
        }
    };

    // 3-レジスタ && 命令引数あり
    ($inst:ident ( $($arg:expr),* ) $dst:expr, $src1:expr, $src2:expr) => {
        LirTree::Inst {
            inst: $inst ( $($arg),* ),
            dst: $dst,
            src1: $src1,
            src2: $src2,
        }
    };

    // 3-レジスタ && 命令引数なし
    ($inst:ident $dst:expr, $src1:expr, $src2:expr) => {
        LirTree::Inst {
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
}

#[derive(Debug)]
pub enum LirTree {
    // 単一で実行されるブロック (func, if, ...)
    Single {
        result_reg: u32,
        lirs: Vec<LirTree>,
    },

    // 多重で実行されるブロック (for, while, ...)
    Multiple {
        lirs: Vec<LirTree>,
    },

    // ラベル
    Label {
        label: u32,
    },

    // 単一命令
    Inst {
        inst: LirInst,
        dst: u32,
        src1: u32,
        src2: u32,
    },
}

impl LirTree {
    pub fn result_reg(&self) -> u32 {
        match self {
            LirTree::Single { result_reg, .. } => *result_reg,
            LirTree::Multiple { .. } => 0,
            _ => panic!("LirTree::result_reg() called on non-node"),
        }
    }
}
