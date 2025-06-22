use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::Span;
use sb_compiler_type::{Primitive, Type, Typed, Void};

use super::{Value, SemCheck, Dep};

#[derive(Debug)]
pub struct Call<'src> {
    pub ident: Span<'src>,
    pub args: Vec<Value<'src>>,
}

impl<'src> SemCheck<Dep<'_, 'src>, ast::Call<'src>> for Call<'src> {
    async fn check0(ctx: Dep<'_, 'src>, call: ast::Call<'src>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let mut args = vec![];
        for arg in call.args {
            args.push(Value::check(ctx, arg).await?);
        }

        Ok(Call { ident: call.ident, args })
    }
}

impl Typed for Call<'_> {
    fn ty(&self) -> &Type {
        &Primitive(Void)
    }
}
