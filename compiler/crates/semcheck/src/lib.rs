use sb_compiler_parse_ast as ast;
use sb_compiler_semcheck_hir::{self as hir, SemCheck};
use sb_compiler_semcheck_async::prelude::*;
use sb_compiler_semcheck_impl::SemCheckServer;
use sb_compiler_utils::error::ErrComposer;

type ASTs<'name, 'src> = Vec<(&'name str, ast::Program<'src>)>;
type HIRs<'src> = Vec<hir::Program<'src>>;

pub fn semcheck<'name, 'src>(asts: ASTs<'name, 'src>) -> miette::Result<HIRs<'src>> {
    let (_, semcheck_ctx) = SemCheckServer::new();
    let semcheck = |ast| {
        hir::Program::check(semcheck_ctx.clone(), ast)
    };

    asts.into_iter()
        .map(|(_, ast)| semcheck(ast))
        .join_all()
        .block_on()
        .compose()
}
