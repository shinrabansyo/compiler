mod utils;

use std::io::Cursor;

use sb_assembler::assemble;
use sb_emulator::Emulator;
use sb_linker::config::Config;
use sb_linker::obj::Object;
use sb_linker::link;

use utils::{Expect, test_dir};
use sb_compiler::compile;

fn test_code(input: &str) -> anyhow::Result<()> {
    // コンパイル
    let objs = compile(input)?;
    let mut buf = vec![];
    Object::dump(&mut buf, &objs)?;
    let objs = Cursor::new(buf);

    // リンク
    let asm = link(Config::default(), vec![objs])?;

    // アセンブル
    let (dmem, imem) = assemble(&asm)?;
    let dmem = dmem
        .lines()
        .map(|line| u8::from_str_radix(line, 16).unwrap())
        .collect::<Vec<_>>();
    let imem = imem
        .lines()
        .map(|line| u8::from_str_radix(line, 16).unwrap())
        .collect::<Vec<_>>();

    // エミュレータ実行
    let mut emu = Emulator::new(0, &dmem, &imem);
    loop {
        let old_pc = emu.pc;
        emu.step()?;
        if old_pc == emu.pc {
            break;
        }
    }

    // main 関数の返り値を確認
    let r10 = emu.regs.read(10)?;
    if r10 == 0 {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Failed: expected {}, but got {}", 0, r10))
    }
}

#[test]
fn success() {
    test_dir("tests/success", Expect::Ok, &test_code);
}

#[test]
fn fail() {
    test_dir("tests/fail", Expect::Err, &test_code);
}
