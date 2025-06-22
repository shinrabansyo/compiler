use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::Span;
use sb_compiler_semcheck_impl_type_decl::r#type::primitive::*;
use sb_compiler_semcheck_impl_type_decl::r#type::*;

use super::{ArgumentDef, Block, SemCheck, InDep};

#[derive(Debug)]
pub struct FuncDef<'src> {
    pub ident: Span<'src>,
    pub args: Vec<ArgumentDef<'src>>,
    pub ret_ty: Option<Span<'src>>,
    pub block: Block<'src>,
}

impl<'src> SemCheck<InDep<'src>, ast::FuncDef<'src>> for FuncDef<'src> {
    async fn check0(mut ctx: InDep<'src>, func_def: ast::FuncDef<'src>) -> anyhow::Result<Self>
        where
            Self: Sized
    {
        ctx.name.push(func_def.ident.as_str());

        let mut args = vec![];
        for arg in func_def.args {
            args.push(ArgumentDef::check(&mut ctx, arg).await?);
        }

        let block = Block::check(ctx.clone(), func_def.block).await?;

        ctx.name.pop();

        Ok(FuncDef {
            ident: func_def.ident,
            args,
            ret_ty: func_def.ret_ty,
            block,
        })
    }

    fn ty(&self) -> &Type {
        &Primitive(Void)
    }
}
