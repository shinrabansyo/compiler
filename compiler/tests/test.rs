mod utils;

use sb_asm::assemble;
use sb_emu::{State, step};

use utils::{Expect, test_dir};
use sb_compiler::compile;

fn test_code(input: &str) -> anyhow::Result<()> {
    // コンパイル
    let asm = compile(input)?;

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
    let mut emu = step(State::new(0, &dmem, &imem))?;
    while emu.pc != 0 {
        emu = step(emu)?;
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
