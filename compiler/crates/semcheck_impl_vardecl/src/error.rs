use thiserror::Error;
use miette::{Diagnostic, SourceSpan};

use sb_compiler_parse_cst::Span;

#[derive(Debug, Error, Diagnostic)]
#[diagnostic(
    code(semantics::variable::declaration),
    url("this link is not working yet"),
)]
pub enum VarError {
    #[error("Variable '{name}' is not declared in this scope.")]
    NotDeclared {
        #[source_code]
        src: String,
        #[label("here")]
        span: SourceSpan,
        name: String,
    },
}

impl VarError {
    pub fn new_not_declared(span: Span) -> miette::Report {
        VarError::NotDeclared {
            src: span.src.to_string(),
            span: span.into(),
            name: span.as_str().to_string(),
        }.into()
    }
}
