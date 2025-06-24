mod borrowed;
mod owned;

pub use borrowed::Span;
pub use owned::SpanOwned;

pub trait Spanned<'src> {
    fn span(&self) -> Span<'src>;
}
