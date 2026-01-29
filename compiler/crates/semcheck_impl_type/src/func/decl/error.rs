use thiserror::Error;
use miette::{Diagnostic, SourceSpan};

use sb_compiler_parse_cst::Span;

#[derive(Debug, Error, Diagnostic)]
#[diagnostic(
    code(semantics::r#type::decl),
    url("this link is not working yet"),
)]
pub enum TypeDeclError {
    #[error("Type '{name}' is already declared.")]
    TypeAlreadyDeclared {
        #[source_code]
        src: String,

        #[label("here")]
        span: SourceSpan,

        name: String,
    },

    #[error("Type '{name}' is not declared.")]
    TypeNotDeclared {
        #[source_code]
        src: String,

        #[label("here")]
        span: SourceSpan,

        name: String,
    },
}

impl TypeDeclError {
    pub fn new_already_declared(span: Span) -> miette::Report {
        TypeDeclError::TypeAlreadyDeclared {
            src: span.src.to_string(),
            span: span.into(),
            name: span.as_str().to_string(),
        }.into()
    }

    pub fn new_not_declared(span: Span) -> miette::Report {
        TypeDeclError::TypeNotDeclared {
            src: span.src.to_string(),
            span: span.into(),
            name: span.as_str().to_string(),
        }.into()
    }
}
