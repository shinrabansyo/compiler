use sb_compiler_parse_ast as ast;
use sb_compiler_semcheck_hir::{self as hir, SemCheckFrom};
use sb_compiler_semcheck_impl::SemCheckServer;
use sb_compiler_semcheck_task::block_on;

pub fn semcheck(program: ast::Program) -> anyhow::Result<hir::Program> {
    let (_, semcheck_ctx) = SemCheckServer::new();
    let hir = block_on(hir::Program::check(semcheck_ctx, program))?;
    Ok(hir)
}
