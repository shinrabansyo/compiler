use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::{Span, Spanned};
use sb_compiler_semcheck_impl_type::decl::ty_register;
use sb_compiler_semcheck_impl_type::op::ty_equals;
use sb_compiler_semcheck_impl_type::parse::{ty_parse_func, ty_parse_type};
use sb_compiler_semcheck_impl_type::{Typed, Type, Void};

use super::{ArgumentDef, Block, Stmt, SemCheck, InDep};

#[derive(Debug)]
pub struct FuncDef<'src> {
    pub span: Span<'src>,
    pub name: String,
    pub args: Vec<ArgumentDef<'src>>,
    pub block: Block<'src>,
}

impl<'src> SemCheck<InDep<'src>, ast::FuncDef<'src>> for FuncDef<'src> {
    async fn check0(mut ctx: InDep<'src>, func_def: ast::FuncDef<'src>) -> miette::Result<Self>
        where
            Self: Sized
    {
        // 名前空間を作成
        ctx.name.push(func_def.ident.as_str());

        // 関数名(フルパス)を用意
        let fn_name = ctx.name.as_str().to_string();

        // 関数の型を登録
        let fn_ty = ty_parse_func(&ctx.r#type, &func_def).await?;
        ty_register(&mut ctx.r#type, func_def.ident, fn_ty).await?;

        // 引数の意味解析
        let mut args = vec![];
        for arg in func_def.args {
            args.push(ArgumentDef::check(&mut ctx, arg).await?);
        }

        // ブロックの意味解析 & 型チェック
        let block = Block::check(ctx.clone(), func_def.block).await?;
        match block.stmts.last() {
            Some(Stmt::Return { r#return, .. }) => {
                let ret_ty = match &func_def.ret_ty {
                    Some(ty) => ty_parse_type(&ctx.r#type, &ty).await?,
                    None => Void.ty(),
                };
                ty_equals(&ret_ty, r#return)?;
            }
            _ => ty_equals(&Void, &block)?,
        };

        // 作成した名前空間を削除
        ctx.name.pop();

        Ok(FuncDef {
            span: func_def.span,
            name: fn_name,
            args,
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
