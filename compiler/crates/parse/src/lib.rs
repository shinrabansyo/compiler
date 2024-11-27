use std::cell::LazyCell;

use copager::ir::Tree;
use copager::Processor;

use sb_compiler_parse_ast::AST;
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

pub fn parse(input: &str) -> anyhow::Result<AST> {
    let cst = PROCESSOR.process::<Tree<_>>(input)?;
    Ok(AST::from(cst))
}