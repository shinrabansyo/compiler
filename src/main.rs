use std::env;
use std::fs::File;
use std::fs;

use sb_compiler::compile;
use sb_linker::obj::Object;

fn main() -> anyhow::Result<()> {
    let args = std::env::args();
    if args.len() < 3 {
        return Err(anyhow::anyhow!("usage: sb-compiler <input> <output>"));
    }

    let path = env::args().nth(1).unwrap();
    let input = fs::read_to_string(&path)?;
    let objs = compile(&input)?;

    let path = env::args().nth(2).unwrap();
    let mut f = File::create(&path)?;
    Object::dump(&mut f, &objs)?;

    Ok(())
}
