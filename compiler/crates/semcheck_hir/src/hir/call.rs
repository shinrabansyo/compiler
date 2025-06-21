use sb_compiler_parse_ast as ast;
use sb_compiler_parse_cst::Span;

use super::{Value, SemCheckFrom, Dep};

#[derive(Debug)]
pub struct Call<'input> {
    pub ident: Span<'input>,
    pub args: Vec<Value<'input>>,
}

impl<'input> SemCheckFrom<Dep<'_>, ast::Call<'input>> for Call<'input> {
    async fn check0(ctx: Dep<'_>, call: ast::Call<'input>) -> anyhow::Result<Self>
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
