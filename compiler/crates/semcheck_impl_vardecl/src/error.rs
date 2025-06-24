use thiserror::Error;
use miette::{Diagnostic, SourceSpan};

use sb_compiler_parse_cst::Span;

#[derive(Debug, Error, Diagnostic)]
#[diagnostic(
    code(semantics::variable::declaration),
    url("this link is not working yet"),
)]
pub enum VarDeclError {
    #[error("Variable '{name}' is not declared in this scope.")]
    NotDeclared {
        #[source_code]
        src: String,
        #[label("here")]
        span: SourceSpan,
        name: String,
    },
}

impl VarDeclError {
    pub fn new_not_declared(span: Span) -> miette::Report {
        VarDeclError::NotDeclared {
            src: span.src().to_string(),
            span: (span.body.0, span.body.1-span.body.0).into(),
            name: span.as_str().to_string(),
        }.into()
    }
}
