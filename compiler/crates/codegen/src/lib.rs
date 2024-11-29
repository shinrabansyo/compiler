use std::fmt::Write;

use sb_compiler_lirgen_ir::LIR;

pub fn codegen(lirs: Vec<LIR>) -> String {
    let dmem = "";

    let mut imem = String::new();
    writeln!(&mut imem, "addi r2 = r0, 0x100").unwrap();
    for lir in lirs {
        writeln!(&mut imem, "{}", lir).unwrap();
    }

    format!("{}\n===\n{}", dmem, imem)
}
