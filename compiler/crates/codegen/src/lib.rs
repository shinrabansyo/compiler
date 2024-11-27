use sb_compiler_lirgen_ir::LIR;

pub fn codegen(lirs: Vec<LIR>) -> Vec<String> {
    let mut asms = vec![];
    codegen_dmem(&mut asms);
    codegen_imem(&mut asms, 0, &lirs);
    asms
}

fn codegen_dmem(asms: &mut Vec<String>) {
    asms.push(format!("==="));
}

fn codegen_imem(asms: &mut Vec<String>, indent: usize, lirs: &[LIR]) {
    macro_rules! asm {
        ($format:expr $(, $arg:expr)*) => {
            asms.push(format!(
                concat!("{}", $format),
                " ".repeat(indent),
                $($arg),*
            ));
        };
    }

    // SP 初期化
    asm!("addi r2 = r0, 0x100");

    // 命令列変換
    for lir in lirs {
        match lir {
            LIR::Li(li) => {
                asm!("addi r{} = r0, {}", li.reg, li.value);
            }
            LIR::Add(add) => {
                asm!("add r{} = r{}, r{}", add.lhs_reg, add.lhs_reg, add.rhs_reg);
            }
            LIR::Sub(sub) => {
                asm!("sub r{} = r{}, r{}", sub.lhs_reg, sub.lhs_reg, sub.rhs_reg);
            }
            LIR::Push(push) => {
                asm!("subi r2 = r2, 4");
                asm!("sw r2[0] = r{}", push.reg);
            }
            LIR::Pop(pop) => {
                asm!("lw r{} = r2[0]", pop.reg);
                asm!("addi r2 = r2, 4");
            }
            LIR::Sw(sw) => {
                asm!("sw r{}[{}] = r{}", sw.base_reg, sw.addr, sw.src_reg);
            }
            LIR::Lw(lw) => {
                asm!("lw r{} = r{}[{}]", lw.dst_reg, lw.base_reg, lw.addr);
            }
        }
    }
}
