use sb_compiler_parse_ast as ast;
use sb_compiler_semcheck_hir::{self as hir, SemCheck};
use sb_compiler_semcheck_async::prelude::*;
use sb_compiler_semcheck_impl::SemCheckContext;
use sb_compiler_utils::error::ErrComposer;

type ASTs<'name, 'src> = Vec<(&'name str, ast::Program<'src>)>;
type HIRs<'src> = Vec<hir::Program<'src>>;

pub fn semcheck<'name, 'src>(asts: ASTs<'name, 'src>) -> miette::Result<HIRs<'src>> {
    let ctx = SemCheckContext::new();
    let semcheck = |(name, ast)| {
        let mut ctx = ctx.clone();
        ctx.name.push(name);
        hir::Program::check(ctx, ast)
    };

    asts.into_iter()
        .map(semcheck)
        .join_all()
        .block_on()
        .compose()
}
