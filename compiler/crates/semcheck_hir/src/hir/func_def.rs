use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::Span;
use sb_compiler_semcheck_impl_typedecl::TypeDeclChecker;
use sb_compiler_type::op::ty_equals;
use sb_compiler_type::r#type::{Primitive, Type, Void};
use sb_compiler_type::Typed;

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
        // 名前空間を作成
        ctx.name.push(func_def.ident.as_str());

        // 引数の意味解析
        let mut args = vec![];
        for arg in func_def.args {
            args.push(ArgumentDef::check(&mut ctx, arg).await?);
        }

        // ブロックの意味解析
        let block = Block::check(ctx.clone(), func_def.block).await?;

        // 戻り値の型チェック
        let ret_type = match &func_def.ret_ty {
            Some(ty) => TypeDeclChecker::find(&ctx.type_decl, ty.as_str()).await?,
            None => Primitive(Void),
        };
        ty_equals(&ret_type, block.ty())?;

        // 作成した名前空間を削除
        ctx.name.pop();

        Ok(FuncDef {
            ident: func_def.ident,
            args,
            ret_ty: func_def.ret_ty,
            block,
        })
    }
}

impl Typed for FuncDef<'_> {
    fn ty(&self) -> &Type {
        &Primitive(Void)
    }
}
