use sb_compiler_parse_ast as ast;
use sb_compiler_semcheck_hir::{self as hir, SemCheck};
use sb_compiler_semcheck_async::prelude::*;
use sb_compiler_semcheck_impl::SemCheckServer;

pub fn semcheck(program: ast::Program) -> anyhow::Result<hir::Program> {
    let (_, semcheck_ctx) = SemCheckServer::new();
    let hir = hir::Program::check(semcheck_ctx, program).block_on()?;
    Ok(hir)
}
