use thiserror::Error;
use miette::{Diagnostic, SourceSpan};

use sb_compiler_parse_cst::Spanned;

use crate::Typed;

#[derive(Debug, Error, Diagnostic)]
#[diagnostic(
    code(semantics::ty::op),
    url("this link is not working yet"),
)]
pub enum TypeOpError {
    #[error("Cannot cast from '{from}' to '{to}'")]
    CastFailed {
        #[source_code]
        src: String,

        #[label("This is '{from}'")]
        from_span: SourceSpan,
        from: String,

        to: String,
    },

    #[error("Type mismatch, '{b}' is not compatible with '{a}'")]
    Mismatch {
        #[source_code]
        src: String,

        #[label("This is '{b}'")]
        b_span: SourceSpan,
        b: String,

        a: String,
    },

    #[error("Type mismatch, '{a}' is not compatible with '{b}'")]
    Mismatch2 {
        #[source_code]
        src: String,

        #[label("This is '{b}'")]
        b_span: SourceSpan,
        b: String,

        a: String,
    },

    #[error("Cannot determine type from '{from}' and '{to}'")]
    Det2Failed {
        #[source_code]
        src: String,

        #[label("This is '{from}'")]
        from_span: SourceSpan,
        from: String,

        #[label("This is '{to}'")]
        to_span: SourceSpan,
        to: String,
    }
}

impl TypeOpError {
    pub fn new_cast_failed<'src, F, T>(from: &F, to: &T) -> miette::Report
    where
        F: Typed + Spanned<'src>,
        T: Typed,
    {
        TypeOpError::CastFailed {
            src: from.span().src.to_string(),
            from_span: from.span().into(),
            from: from.ty().to_string(),
            to: to.ty().to_string(),
        }.into()
    }

    pub fn new_mismatch<'src, A, B>(a: &A, b: &B) -> miette::Report
    where
        A: Typed,
        B: Typed + Spanned<'src>,
    {
        TypeOpError::Mismatch {
            src: b.span().src.to_string(),
            b_span: b.span().into(),
            b: b.ty().to_string(),
            a: a.ty().to_string(),
        }.into()
    }

    pub fn new_mismatch2<'src, A, B>(a: &A, b: &B) -> miette::Report
    where
        A: Typed,
        B: Typed + Spanned<'src>,
    {
        TypeOpError::Mismatch2 {
            src: b.span().src.to_string(),
            a: a.ty().to_string(),
            b_span: b.span().into(),
            b: b.ty().to_string(),
        }.into()
    }

    pub fn new_det2_failed<'src, F, T>(from: &F, to: &T) -> miette::Report
    where
        F: Typed + Spanned<'src>,
        T: Typed + Spanned<'src>,
    {
        TypeOpError::Det2Failed {
            src: from.span().src.to_string(),
            from_span: from.span().into(),
            from: from.ty().to_string(),
            to_span: to.span().into(),
            to: to.ty().to_string(),
        }.into()
    }
}
