mod utils;

use utils::{Expect, test_dir};

use sb_compiler_parse::parse;

#[test]
fn success() {
    test_dir("tests/success", Expect::Ok, &parse);
}

#[test]
fn fail() {
    test_dir("tests/fail", Expect::Err, &parse);
}
