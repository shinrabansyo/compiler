use std::cell::LazyCell;

use copager::Processor;

use sb_compiler_parse_ast::Program;
use sb_compiler_parse_cst::{CSTree, CSTreeVisitor};
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

pub fn parse(input: &str) -> miette::Result<Program> {
    let cst = match PROCESSOR.process::<CSTree<_>>(input) {
        Ok(cst) => cst,
        Err(err) => return Err(miette::miette!("Failed to parse input: {}", err)),
    };

    let visitor = CSTreeVisitor::from(cst);
    let ast = Program::from(visitor);
    Ok(ast)
}
