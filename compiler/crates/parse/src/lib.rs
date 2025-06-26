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

type Texts<'name, 'src> = Vec<(&'name str, &'src str)>;
type ASTs<'name, 'src> = Vec<(&'name str, Program<'src>)>;

pub fn parse<'name, 'src>(inputs: Texts<'name, 'src>) -> miette::Result<ASTs<'name, 'src>> {
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
        .into_iter()
        .map(|(name, input)| (name, parse(input)))
        .map(|(name, ast)| ast.map(|ast| (name, ast)))
        .compose()
}
