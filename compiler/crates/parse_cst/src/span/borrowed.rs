use std::fmt::{Debug, Display};

use copager::cfl::token::Token;
use copager::cfl::token::TokenTag;
use miette::SourceSpan;

use super::SpanOwned;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span<'a> {
    pub src: &'a str,
    pub body: (usize, usize),   // Trivia を含まない
    pub full: (usize, usize),   // Trivia を含む
}

impl Debug for Span<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Display for Span<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a, T> From<Token<'a, T>> for Span<'a>
where
    T: TokenTag,
{
    fn from(token: Token<'a, T>) -> Self {
        Span {
            src: token.src,
            body: token.body,
            full: token.full,
        }
    }
}

impl<'a> Into<SourceSpan> for Span<'a> {
    fn into(self) -> SourceSpan {
        (self.body.0, self.body.1 - self.body.0).into()
    }
}

impl<'a> Span<'a> {
    pub fn as_str(&self) -> &'a str {
        let (l, r) = self.body;
        &self.src[l..r]
    }

    pub fn as_full_str(&self) -> &'a str {
        let (l, r) = self.full;
        &self.src[l..r]
    }

    pub fn to_owned(&self) -> SpanOwned {
        SpanOwned {
            src: self.src.to_string(),
            body: self.body,
            full: self.full,
        }
    }
}
