use sb_compiler_lirgen_ir::LIR;

pub fn codegen(lirs: Vec<LIR>) -> Vec<String> {
    let mut asms = vec![];
    codegen_dmem(&mut asms);
    codegen_imem(&mut asms, &lirs);
    asms
}

fn codegen_dmem(asms: &mut Vec<String>) {
    asms.push(format!("==="));
}

fn codegen_imem(asms: &mut Vec<String>, lirs: &[LIR]) {
    macro_rules! asm {
        ($format:expr $(, $arg:expr)*) => {
            asms.push(format!($format, $($arg),*));
        };
    }

    // SP 初期化
    asm!("addi r2 = r0, 0x100");

    // 命令列変換
    for idx in 0..lirs.len() {
        let lir = &lirs[idx];
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
            LIR::Jmp(jmp) => {
                asm!("beq r0, (r0, r0) -> @{}", jmp.label);
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
            LIR::Label(label) => {
                asm!("@{}", label.name);
            }
            LIR::FSave(_) => {
                asm!("subi r2 = r2, 4");
                asm!("sw r2[0] = r3");
                asm!("add r3 = r0, r2");

                asm!("subi r2 = r2, 44");
                asm!("sw r3[-4] = r1");
                asm!("sw r3[-8] = r20");
                asm!("sw r3[-16] = r21");
                asm!("sw r3[-20] = r22");
                asm!("sw r3[-24] = r23");
                asm!("sw r3[-28] = r24");
                asm!("sw r3[-32] = r25");
                asm!("sw r3[-36] = r26");
                asm!("sw r3[-40] = r27");
                asm!("sw r3[-44] = r28");
                asm!("sw r3[-48] = r29");
            }
            LIR::FLoad(_) => {
                asm!("lw r1 = r3[-4]");
                asm!("lw r20 = r3[-8]");
                asm!("lw r21 = r3[-16]");
                asm!("lw r22 = r3[-20]");
                asm!("lw r23 = r3[-24]");
                asm!("lw r24 = r3[-28]");
                asm!("lw r25 = r3[-32]");
                asm!("lw r26 = r3[-36]");
                asm!("lw r27 = r3[-40]");
                asm!("lw r28 = r3[-44]");
                asm!("lw r29 = r3[-48]");
                asm!("addi r2 = r2, 44");

                asm!("lw r3 = r2[0]");
                asm!("addi r2 = r2, 4");
            }
            LIR::Return(_) => {
                asm!("jal r0, r1[0]");
            }
        }
    }
}
