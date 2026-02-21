pub mod syntax;
mod translate;
mod var;

use syntax::LirSyntax;
use translate::{Translatable, TranslateContext};

#[derive(Debug)]
pub struct Lir<T, Next>
where
    T: LirSyntax,
    Next: LirSyntax,
{
    syntax: T,
    next: Option<Next>,
}

impl<T, Next> Translatable for Lir<T, Next>
where
    T: LirSyntax,
    Next: LirSyntax,
{
    fn translate(&mut self, ctx: &mut TranslateContext) {
        self.syntax.translate(ctx);
        self.next.as_mut().unwrap().translate(ctx);
    }
}

impl<T, Next> Lir<T, Next>
where
    T: LirSyntax,
    Next: LirSyntax,
{
    pub fn wrap(syntax: T) -> Self {
        Lir { syntax, next: None }
    }

    pub fn chain(mut self, next: Next) -> Self {
        self.next.replace(next);
        self
    }

    pub fn next(self) -> Next {
        self.next.unwrap()
    }
}
