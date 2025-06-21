use copager::cfl::token::Token;
use copager::cfl::token::TokenTag;

use super::SpanOwned;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span<'a> {
    pub src: &'a str,
    pub body: (usize, usize),   // Trivia を含まない
    pub full: (usize, usize),   // Trivia を含む
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

impl<'a> Span<'a> {
    pub fn src(&self) -> &'a str {
        self.src
    }

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
