pub struct Block {
    pub id: u32,
    pub stmts: Vec<Stmt>,
}

pub enum Stmt {
    Assign {
        inst: Inst,
        dst: u32,
        src1: u32,
        src2: u32,
    },
    AssignPhi {
        dst: u32,
        srcs: Vec<u32>,
    },
}

pub enum Inst {
    // 算術演算 (即値あり)
    Li(i32),

    // 算術演算 (即値なし)
    Add,
}
