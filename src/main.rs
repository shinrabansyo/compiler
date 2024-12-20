use sb_compiler::compile;

fn main() -> anyhow::Result<()> {
    let args = std::env::args();
    if args.len() < 3 {
        return Err(anyhow::anyhow!("usage: sb-compiler <input> <output>"));
    }

    let path = std::env::args().nth(1).unwrap();
    let input = std::fs::read_to_string(&path)?;
    let asm = compile(&input)?;

    let path = std::env::args().nth(2).unwrap();
    std::fs::write(&path, asm)?;

    Ok(())
}
