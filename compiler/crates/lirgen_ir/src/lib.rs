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
    Node {
        reserved_reg_range: (u32, u32),
        reserved_label_range: (u32, u32),
        result_reg: u32,
        lirs: Vec<LirTree>,
    },
    Label {
        label: u32,
    },
    Inst {
        inst: LirInst,
        dst: u32,
        src1: u32,
        src2: u32,
    },
}

impl LirTree {
    pub fn reserved_reg_range(&self) -> (u32, u32) {
        match self {
            LirTree::Node { reserved_reg_range, .. } => *reserved_reg_range,
            _ => panic!("LirTree::reserved_regs() called on non-node"),
        }
    }

    pub fn reserved_label_range(&self) -> (u32, u32) {
        match self {
            LirTree::Node { reserved_label_range, .. } => *reserved_label_range,
            _ => panic!("LirTree::reserved_labels() called on non-node"),
        }
    }

    pub fn result_reg(&self) -> u32 {
        match self {
            LirTree::Node { result_reg, .. } => *result_reg,
            _ => panic!("LirTree::result_reg() called on non-node"),
        }
    }
}
