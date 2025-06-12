use std::cell::LazyCell;

use copager::Processor;

use sb_compiler_parse_ast::Program;
use sb_compiler_parse_cst::CSTreeVisitor;
use sb_compiler_parse_syntax::SBLang;

const PROCESSOR: LazyCell<Processor<SBLang>> = LazyCell::new(|| {
    #[copager::load]
    fn loader(processor: Processor<SBLang>) -> Processor<SBLang> {
        processor
            .build_lexer()
            .unwrap()
            .restore_parser_by_cache()
    }
    loader()
});

pub fn parse(input: &str) -> anyhow::Result<Program> {
    let visitor = PROCESSOR.process::<CSTreeVisitor<_>>(input)?;
    let ast = Program::from(("".into(), visitor));
    Ok(ast)
}
