use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_semcheck_impl_type::decl::ty_register_in_mod;
use sb_compiler_type::op::ty_equals;
use sb_compiler_type::r#type::{Function, Type, Void};
use sb_compiler_type::Typed;

use super::{ArgumentDef, Block, Stmt, SemCheck, InDep};

#[derive(Debug)]
pub struct FuncDef<'src> {
    pub span: Span<'src>,
    pub name: String,
    pub args: Vec<ArgumentDef<'src>>,
    pub ret_ty: Arc<Type>,
    pub block: Block<'src>,
}

impl<'src> SemCheck<InDep<'src>, ast::FuncDef<'src>> for FuncDef<'src> {
    async fn check0(mut ctx: InDep<'src>, func_def: ast::FuncDef<'src>) -> miette::Result<Self>
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

        // 関数の型を登録
        let arg_tys = args
            .iter()
            .map(|arg| arg.ty())
            .collect::<Vec<_>>();
        let ret_ty = match func_def.ret_ty {
            // Some(ty) => TypeDeclChecker::find(&ctx.type_decl, &ty).await?,
            Some(ty) => Type::from(ty).ty(),
            None => Void.ty(),
        };
        let fn_ty = Arc::new(Function {
            args: arg_tys,
            ret_ty: ret_ty.ty(),
        });
        let fn_name = ty_register_in_mod(
            &mut ctx.r#type,
            ctx.name.as_parent_str(),
            func_def.ident,
            fn_ty,
        ).await?;

        // ブロックの意味解析 & 型チェック
        let block = Block::check(ctx.clone(), func_def.block).await?;
        match block.stmts.last() {
            Some(Stmt::Return { r#return, .. }) => {
                let ret_ty = ret_ty.as_ref().clone();
                ty_equals(ret_ty, r#return)?;
            }
            _ => ty_equals(Void, &block)?,
        };

        // 作成した名前空間を削除
        ctx.name.pop();

        Ok(FuncDef {
            span: func_def.span,
            name: fn_name,
            args,
            ret_ty,
            block,
        })
    }
}

impl<'src> Spanned<'src> for FuncDef<'src> {
    fn span(&self) -> Span<'src> {
        self.span
    }
}

impl Typed for FuncDef<'_> {
    fn ty(&self) -> Arc<Type> {
        Void.ty()
    }
}
