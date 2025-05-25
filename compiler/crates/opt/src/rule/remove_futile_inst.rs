use sb_linker::obj::inst::Inst;
use sb_linker::obj::Object;

pub fn remove_futile_inst(mut obj: Object) -> Object {
    fn filter(inst: &Inst) -> bool {
        match inst {
            // R-形式 (rs1 = r0)
            Inst::Add { rd, rs1: 0, rs2 } if rd == rs2 => false,
            Inst::Sub { rd, rs1: 0, rs2 } if rd == rs2 => false,
            Inst::And { rd, rs1: 0, rs2 } if rd == rs2 => false,
            Inst::Or  { rd, rs1: 0, rs2 } if rd == rs2 => false,
            Inst::Xor { rd, rs1: 0, rs2 } if rd == rs2 => false,
            Inst::Sll { rd, rs1: 0, rs2 } if rd == rs2 => false,
            Inst::Srl { rd, rs1: 0, rs2 } if rd == rs2 => false,
            Inst::Sra { rd, rs1: 0, rs2 } if rd == rs2 => false,

            // R-形式 (rs2 = r0)s
            Inst::Add { rd, rs1, rs2: 0 } if rd == rs1 => false,
            Inst::Sub { rd, rs1, rs2: 0 } if rd == rs1 => false,
            Inst::And { rd, rs1, rs2: 0 } if rd == rs1 => false,
            Inst::Or  { rd, rs1, rs2: 0 } if rd == rs1 => false,
            Inst::Xor { rd, rs1, rs2: 0 } if rd == rs1 => false,
            Inst::Sll { rd, rs1, rs2: 0 } if rd == rs1 => false,
            Inst::Srl { rd, rs1, rs2: 0 } if rd == rs1 => false,
            Inst::Sra { rd, rs1, rs2: 0 } if rd == rs1 => false,

            _ => true,
        }
    }

    obj.code = obj.code
        .into_iter()
        .filter(filter)
        .collect();

    obj
}
