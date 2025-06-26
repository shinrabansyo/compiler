use sb_compiler_parse_ast as ast;
use sb_compiler_semcheck_hir::{self as hir, SemCheck};
use sb_compiler_semcheck_async::prelude::*;
use sb_compiler_semcheck_impl::SemCheckServer;
use sb_compiler_utils::error::ErrComposer;

pub fn semcheck<'name, 'src, I>(asts: I) -> miette::Result<Vec<hir::Program<'src>>>
where
    I: Iterator<Item = (&'name str, ast::Program<'src>)>,
{
    let (_, semcheck_ctx) = SemCheckServer::new();
    let semcheck = |ast| {
        hir::Program::check(semcheck_ctx.clone(), ast)
    };

    asts.map(|(_, ast)| semcheck(ast))
        .join_all()
        .block_on()
        .compose()
}
