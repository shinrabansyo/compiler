use copager::Processor;

use sb_compiler_parse_syntax::SBLang;

#[copager::prebuild]
fn main() -> Processor<SBLang> {
    Processor::<SBLang>::new()
        .prebuild_parser()
        .unwrap()
}
