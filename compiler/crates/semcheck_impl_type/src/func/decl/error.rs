use thiserror::Error;
use miette::{Diagnostic, SourceSpan};

use sb_compiler_parse_cst::Span;

#[derive(Debug, Error, Diagnostic)]
#[diagnostic(
    code(semantics::r#type::declaration),
    url("this link is not working yet"),
)]
pub enum TypeDeclError {
    #[error("'{name}' is already declared.")]
    TypeAlreadyDeclared {
        #[source_code]
        src: String,
        #[label("here")]
        span: SourceSpan,
        name: String,
    },
    #[error("'{name}' is not declared.")]
    TypeNotDeclared {
        #[source_code]
        src: String,
        #[label("here")]
        span: SourceSpan,
        name: String,
    },
    #[error("'{name}' is not declared in this scope.")]
    TypeNotDeclaredInScope {
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

    pub fn new_not_declared_in_scope(span: Span) -> miette::Report {
        TypeDeclError::TypeNotDeclaredInScope {
            src: span.src.to_string(),
            span: span.into(),
            name: span.as_str().to_string(),
        }.into()
    }
}
