use std::sync::Arc;

use thiserror::Error;
use miette::{Diagnostic, SourceSpan};

use sb_compiler_parse_cst::Spanned;

use crate::r#type::Type;
use crate::Typed;

#[derive(Debug, Error, Diagnostic)]
#[diagnostic(
    code(semantics::ty::op),
    url("this link is not working yet"),
)]
pub enum TypeOpError {
    #[error("Cannot cast from '{from:?}' to '{to:?}'")]
    CastFailed {
        #[source_code]
        src: String,

        #[label("This is '{from:?}'")]
        from_span: SourceSpan,
        from: Arc<Type>,

        to: Arc<Type>,
    },

    #[error("Type mismatch, '{a:?}' is not compatible with '{ty:?}'")]
    Mismatch {
        #[source_code]
        src: String,

        #[label("This is '{a:?}'")]
        a_span: SourceSpan,
        a: Arc<Type>,

        ty: Type,
    },

    #[error("Type mismatch, '{a:?}' is not compatible with '{b:?}'")]
    Mismatch2 {
        #[source_code]
        src: String,

        #[label("This is '{a:?}'")]
        a_span: SourceSpan,
        a: Arc<Type>,

        #[label("This is '{b:?}'")]
        b_span: SourceSpan,
        b: Arc<Type>,
    },

    #[error("Cannot determine type from '{from:?}' and '{to:?}'")]
    Det2Failed {
        #[source_code]
        src: String,

        #[label("This is '{from:?}'")]
        from_span: SourceSpan,
        from: Type,

        #[label("This is '{to:?}'")]
        to_span: SourceSpan,
        to: Type,
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
            from: from.ty(),
            to: to.ty(),
        }.into()
    }

    pub fn new_mismatch<'src, A>(ty: Type, a: &A) -> miette::Report
    where
        A: Typed + Spanned<'src>,
    {
        TypeOpError::Mismatch {
            src: a.span().src.to_string(),
            a_span: a.span().into(),
            a: a.ty(),
            ty,
        }.into()
    }

    pub fn new_mismatch2<'src, A, B>(a: &A, b: &B) -> miette::Report
    where
        A: Typed + Spanned<'src>,
        B: Typed + Spanned<'src>,
    {
        TypeOpError::Mismatch2 {
            src: a.span().src.to_string(),
            a_span: a.span().into(),
            a: a.ty(),
            b_span: b.span().into(),
            b: b.ty(),
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
            from: from.ty().as_ref().clone(),
            to_span: to.span().into(),
            to: to.ty().as_ref().clone(),
        }.into()
    }
}
