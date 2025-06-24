use std::sync::Arc;

use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::Span;
use sb_compiler_type::r#type::{Primitive, Type, Void};
use sb_compiler_type::Typed;

use super::{Stmt, SemCheck, InDep};

#[derive(Debug)]
pub struct Block<'src> {
    pub span: Span<'src>,
    pub stmts: Vec<Stmt<'src>>,
    pub ty: Arc<Type>,
}

impl<'src> SemCheck<InDep<'src>, ast::Block<'src>> for Block<'src> {
    async fn check0(mut ctx: InDep<'src>, block: ast::Block<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        // 文を順に意味解析
        let mut stmts = vec![];
        for stmt in block.stmts {
            stmts.push(Stmt::check(&mut ctx, stmt).await?);
        }

        // ブロックの型は Void
        let ty = Arc::new(Primitive(Void));

        Ok(Block { span: block.span, stmts, ty })
    }
}

impl Typed for Block<'_> {
    fn ty(&self) -> &Arc<Type> {
        &self.ty
    }
}
