mod utils;

use utils::{Expect, test_dir};

use sb_compiler::compile;

#[test]
fn success() {
    test_dir("tests/success", Expect::Ok, &compile);
}

#[test]
fn fail() {
    test_dir("tests/fail", Expect::Err, &compile);
}
