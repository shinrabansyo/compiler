mod utils;

use copager::ir::Void;
use copager::Processor;

use utils::{Expect, test_dir};

use sb_compiler_parse_syntax::SBLang;

#[test]
fn success() {
    test_dir("tests/success", Expect::Ok, &parse);
}

#[test]
fn fail() {
    test_dir("tests/fail", Expect::Err, &parse);
}

fn parse(input: &str) -> anyhow::Result<()> {
    Processor::<SBLang>::new()
        .build_lexer()?
        .build_parser()?
        .process::<Void>(input)
        .and_then(|_| Ok(()))
}
