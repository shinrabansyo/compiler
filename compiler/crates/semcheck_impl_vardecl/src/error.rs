use thiserror::Error;
use miette::{Diagnostic, SourceSpan};

use sb_compiler_parse_cst::Span;

#[derive(Debug, Error, Diagnostic)]
pub enum VarDeclError {
    #[error("Variable is not declared in this scope.")]
    NotDeclared {
        #[source_code]
        src: String,
        #[label("here")]
        span: SourceSpan,
    },
}

impl VarDeclError {
    pub fn new_not_declared(span: Span) -> miette::Report {
        VarDeclError::NotDeclared {
            src: span.src().to_string(),
            span: (span.body.0, span.body.1-span.body.0).into(),
        }.into()
    }
}
