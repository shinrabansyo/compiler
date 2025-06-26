use std::env;
use std::fs::File;
use std::fs;

use miette::IntoDiagnostic;
use sb_compiler::compile;
use sb_linker::obj::Object;

fn main() -> miette::Result<()> {
    let args = std::env::args();
    if args.len() < 3 {
        return Err(miette::miette!("usage: sb-compiler <input> <output>"));
    }

    let path = env::args().nth(1).unwrap();
    let input = fs::read_to_string(&path).into_diagnostic()?;
    let objs = compile(vec![("main", &input)])?;

    let path = env::args().nth(2).unwrap();
    let mut f = File::create(&path).into_diagnostic()?;
    Object::dump(&mut f, &objs).unwrap();

    Ok(())
}
