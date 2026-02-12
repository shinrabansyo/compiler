mod assign;     pub use assign::Assign;
mod terminal;   pub use terminal::Terminal;

use crate::var::LirVarIssuer;

pub trait LirSyntax
where
    Self: Sized,
{
    fn process(&self, ctx: &mut LirVarIssuer);
    fn is_terminal(&self) -> bool { false }
}

#[derive(Debug)]
pub struct LirSyntaxNode<T, Next>
where
    T: LirSyntax,
    Next: LirSyntax,
{
    syntax: T,
    next: Option<Next>,
}

impl<T, Next> AsRef<T> for LirSyntaxNode<T, Next>
where
    T: LirSyntax,
    Next: LirSyntax,
{
    fn as_ref(&self) -> &T {
        &self.syntax
    }
}

impl<T, Next> LirSyntax for LirSyntaxNode<T, Next>
where
    T: LirSyntax,
    Next: LirSyntax,
{
    fn process(&self, ctx: &mut LirVarIssuer) {
        self.as_ref().process(ctx);
        if let Some(next) = &self.next {
            if next.is_terminal() {
                return;
            }
            next.process(ctx);
        }
    }
}

impl<T, Next> LirSyntaxNode<T, Next>
where
    T: LirSyntax,
    Next: LirSyntax,
{
    pub fn wrap(syntax: T) -> Self {
        LirSyntaxNode { syntax, next: None }
    }

    pub fn chain(mut self, next: Next) -> Self {
        self.next.replace(next);
        self
    }

    pub fn next(self) -> Next {
        self.next.unwrap()
    }
}
