use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::Span;

use super::{ArgumentDef, Block, SemCheckFrom, InDep};

#[derive(Debug)]
pub struct FuncDef<'src> {
    pub ident: Span<'src>,
    pub args: Vec<ArgumentDef<'src>>,
    pub ret_ty: Option<Span<'src>>,
    pub block: Block<'src>,
}

impl<'src> SemCheckFrom<InDep<'src>, ast::FuncDef<'src>> for FuncDef<'src> {
    async fn check0(mut ctx: InDep<'src>, func_def: ast::FuncDef<'src>) -> anyhow::Result<Self>
        where
            Self: Sized
    {
        ctx.push_namespace(func_def.ident.as_str());

        let mut args = vec![];
        for arg in func_def.args {
            args.push(ArgumentDef::check(&mut ctx, arg).await?);
        }

        let block = Block::check(ctx.clone(), func_def.block).await?;

        ctx.pop_namespace();

        Ok(FuncDef {
            ident: func_def.ident,
            args,
            ret_ty: func_def.ret_ty,
            block,
        })
    }
}
