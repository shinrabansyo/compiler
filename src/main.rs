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
    input: Vec<PathBuf>,
}

fn main() -> miette::Result<()> {
    let opts = cli_options().to_options().run();

    let inputs = opts
        .input
        .iter()
        .map(|path| fs::read_to_string(&path).into_diagnostic())
        .collect::<miette::Result<Vec<_>>>()?;
    let inputs = inputs
        .iter()
        .map(|src| ("main", src.as_str()))
        .collect();
    let objs = compile(inputs)?;

    let mut f = File::create(&opts.output).into_diagnostic()?;
    Object::dump(&mut f, &objs).unwrap();

    Ok(())
}
