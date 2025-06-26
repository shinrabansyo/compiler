use std::cell::LazyCell;

use copager::Processor;

use sb_compiler_parse_ast::Program;
use sb_compiler_parse_cst::{CSTree, CSTreeVisitor};
use sb_compiler_parse_syntax::SBLang;
use sb_compiler_utils::error::ErrComposer;

const PARSER: LazyCell<Processor<SBLang>> = LazyCell::new(|| {
    #[copager::load]
    fn loader(processor: Processor<SBLang>) -> Processor<SBLang> {
        processor
            .build_lexer()
            .unwrap()
            .restore_parser_by_cache()
    }
    loader()
});

pub fn parse<'name, 'src, I>(inputs: I) -> miette::Result<Vec<(&'name str, Program<'src>)>>
where
    I: Iterator<Item = (&'name str, &'src str)>,
{
    let parse = |input| {
        let cst = match PARSER.process::<CSTree<_>>(input) {
            Ok(cst) => cst,
            Err(err) => return Err(miette::miette!("Failed to parse input: {}", err)),
        };
        let visitor = CSTreeVisitor::from(cst);
        let ast = Program::from(visitor);
        Ok(ast)
    };

    inputs
        .map(|(name, input)| (name, parse(input)))
        .map(|(name, ast)| ast.map(|ast| (name, ast)))
        .compose()
}
