mod utils;

use std::fmt::Write;

use sb_compiler_parse::parse;
use sb_compiler_lirgen::lirgen;
use sb_compiler_lirgen_ir::{LirInst, LirBlock, LirTopElem};
use utils::{Expect, test_dir};

fn display_lir(f: &mut String, lir: &LirBlock) -> std::fmt::Result {
    match lir {
        LirBlock::Single { lirs, .. } => {
            for lir in lirs {
                display_lir(f, lir)?;
            }
            Ok(())
        }
        LirBlock::Multiple { lirs } => {
            for lir in lirs {
                display_lir(f, lir)?;
            }
            Ok(())
        }
        LirBlock::Label { label } => {
            writeln!(f, "@local.{}", label)
        }
        LirBlock::Inst { inst, dst, src1, src2 } => {
            write!(f, "    ")?;
            match inst {
                LirInst::Nop => {
                    writeln!(f, "nop")
                }
                LirInst::Li(imm) => {
                    writeln!(f, "li   t{} = {}", dst, imm)
                }
                LirInst::Addi(imm) => {
                    writeln!(f, "addi t{} = t{} + {}", dst, src1, imm)
                }
                LirInst::Subi(imm) => {
                    writeln!(f, "subi t{} = t{} - {}", dst, src1, imm)
                }
                LirInst::Andi(imm) => {
                    writeln!(f, "andi t{} = t{} & {}", dst, src1, imm)
                }
                LirInst::Ori(imm) => {
                    writeln!(f, "ori  t{} = t{} | {}", dst, src1, imm)
                }
                LirInst::Xori(imm) => {
                    writeln!(f, "xori t{} = t{} ^ {}", dst, src1, imm)
                }
                LirInst::ShiftLi(imm) => {
                    writeln!(f, "slli t{} = t{} << {}", dst, src1, imm)
                }
                LirInst::ShiftRi(imm) => {
                    writeln!(f, "srli t{} = t{} >> {}", dst, src1, imm)
                }
                LirInst::ShiftRai(imm) => {
                    writeln!(f, "srai t{} = t{} >> {}", dst, src1, imm)
                }
                LirInst::Add => {
                    writeln!(f, "add  t{} = t{} + t{}", dst, src1, src2)
                }
                LirInst::Sub => {
                    writeln!(f, "sub  t{} = t{} - t{}", dst, src1, src2)
                }
                LirInst::And => {
                    writeln!(f, "and  t{} = t{} & t{}", dst, src1, src2)
                }
                LirInst::Or => {
                    writeln!(f, "or   t{} = t{} | t{}", dst, src1, src2)
                }
                LirInst::Xor => {
                    writeln!(f, "xor  t{} = t{} ^ t{}", dst, src1, src2)
                }
                LirInst::ShiftL => {
                    writeln!(f, "sll  t{} = t{} << t{}", dst, src1, src2)
                }
                LirInst::ShiftR => {
                    writeln!(f, "srl  t{} = t{} >> t{}", dst, src1, src2)
                }
                LirInst::ShiftRa => {
                    writeln!(f, "sra  t{} = t{} >> t{}", dst, src1, src2)
                }
                LirInst::Beq(imm) => {
                    writeln!(f, "beq  t{}, (t{} == t{}) -> {}", dst, src1, src2, imm)
                }
                LirInst::Bne(imm) => {
                    writeln!(f, "bne  t{}, (t{} != t{}) -> {}", dst, src1, src2, imm)
                }
                LirInst::Blt(imm) => {
                    writeln!(f, "blt  t{}, (t{} < t{}) -> {}", dst, src1, src2, imm)
                }
                LirInst::Ble(imm) => {
                    writeln!(f, "ble  t{}, (t{} <= t{}) -> {}", dst, src1, src2, imm)
                }
                LirInst::Jmp(imm) => {
                    writeln!(f, "jmp  t{}, {}", dst, imm)
                }
                LirInst::JmpLabel(label) => {
                    writeln!(f, "jmp  t{}, @local.{}", dst, label)
                }
                LirInst::Call(func) => {
                    writeln!(f, "call {}", func)
                }
                LirInst::FnPrologue => {
                    writeln!(f, "fn_prologue")
                }
                LirInst::FnEpilogue => {
                    writeln!(f, "fn_epilogue")
                }
                LirInst::FnReturn => {
                    writeln!(f, "fn_return")
                }
            }
        }
    }
}

fn test_code(input: &str) -> anyhow::Result<String> {
    // 1. 構文解析 (&str -> AST)
    let ast = parse(input)?;

    // 2. LIR生成 (AST -> LIR)
    let lirs = lirgen(&ast);

    // 3. 文字列へ変換 (LIR -> String)
    let mut lir_str = String::new();
    for lir in &lirs {
        let lir_block = match lir {
            LirTopElem::Function { body, .. } => body,
        };
        display_lir(&mut lir_str, lir_block)?;
    }

    Ok(lir_str)
}

#[test]
fn success() {
    test_dir("tests/success", Expect::Ok, &test_code);
}

#[test]
fn fail() {
    test_dir("tests/fail", Expect::Err, &test_code);
}
