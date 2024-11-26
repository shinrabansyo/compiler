use copager::ir::Tree;
use copager::Processor;

use sb_compiler_parse_ast::Expr;
use sb_compiler_parse_syntax::SBLang;

pub fn parse(input: &str) -> anyhow::Result<Expr> {
    let cst = processor_loader().process::<Tree<_>>(input)?;
    Ok(Expr::from(cst))
}

#[copager::load]
fn processor_loader(processor: Processor<SBLang>) -> Processor<SBLang> {
    processor
        .build_lexer()
        .unwrap()
        .restore_parser_by_cache()
}
