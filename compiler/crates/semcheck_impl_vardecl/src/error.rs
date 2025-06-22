use std::error::Error;
use std::fmt::Display;

use sb_compiler_parse_cst::{Span, SpanOwned};

#[derive(Debug)]
pub enum VarDeclError {
    NotDeclared(SpanOwned),
}

impl VarDeclError {
    pub fn new_not_declared(span: Span) -> anyhow::Error {
        VarDeclError::NotDeclared(span.to_owned()).into()
    }
}

impl Error for VarDeclError {}

impl Display for VarDeclError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VarDeclError::NotDeclared(span) => {
                writeln!(f, "Variable '{}' is not declared in this scope.", span.as_str())?;
                span.pretty_display(f)
            }
        }
    }
}
