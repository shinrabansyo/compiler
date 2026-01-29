use thiserror::Error;
use miette::{Diagnostic, SourceSpan};

use sb_compiler_parse_cst::Span;

use crate::Typed;

#[derive(Debug, Error, Diagnostic)]
#[diagnostic(
    code(semantics::r#type::r#struct),
    url("this link is not working yet"),
)]
pub enum TypeStructError {
    #[error("Type '{a}' is not a struct type")]
    NotStruct {
        a: String,
    },

    #[error("Type '{struct}' has no field named '{field}'")]
    FieldNotExists {
        #[source_code]
        src: String,

        #[label("here")]
        field_span: SourceSpan,
        field: String,

        r#struct: String,
    },
}

impl TypeStructError {
    pub fn new_not_struct<'src, A>(a: &A) -> miette::Report
    where
        A: Typed,
    {
        TypeStructError::NotStruct {
            a: a.ty().to_string(),
        }.into()
    }

    pub fn new_field_not_exists<'src, S>(
        r#struct: &S,
        field: Span<'_>,
    ) -> miette::Report
    where
        S: Typed,
    {
        TypeStructError::FieldNotExists {
            src: field.src.into(),
            field_span: field.into(),
            field: field.to_string(),
            r#struct: r#struct.ty().to_string(),
        }.into()
    }
}

