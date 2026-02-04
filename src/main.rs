use std::fs::File;
use std::fs;
use std::path::PathBuf;

use bpaf::{Bpaf, Parser};
use miette::IntoDiagnostic;

use sb_compiler::compile;
use sb_linker::obj::Object;

#[derive(Bpaf)]
struct CliOptions {
    /// Output file
    #[bpaf(short, long, fallback("a.obj".into()))]
    output: PathBuf,
    /// Input file
    #[bpaf(positional("INPUT"))]
    input: PathBuf,
}

fn main() -> miette::Result<()> {
    let opts = cli_options().to_options().run();

    let input = fs::read_to_string(&opts.input).into_diagnostic()?;
    let objs = compile(vec![("main", &input)])?;
    let mut f = File::create(&opts.output).into_diagnostic()?;
    Object::dump(&mut f, &objs).unwrap();

    Ok(())
}
