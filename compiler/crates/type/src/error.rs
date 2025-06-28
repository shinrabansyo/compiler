use std::sync::Arc;

use thiserror::Error;
use miette::{Diagnostic, SourceSpan};

use sb_compiler_parse_cst::Spanned;

use crate::r#type::Type;
use crate::Typed;

#[derive(Debug, Error, Diagnostic)]
#[diagnostic(
    code(semantics::ty::check),
    url("this link is not working yet"),
)]
pub enum TypeError {
    #[error("Cannot cast from type '{from:?}' to '{to:?}'")]
    CastFailed {
        #[source_code]
        src: String,

        #[label("This is type of '{from:?}'")]
        from_span: SourceSpan,
        from: Arc<Type>,

        to: Arc<Type>,
    },

    #[error("Type mismatch, '{a:?}' is not compatible with '{ty:?}'")]
    Mismatch {
        #[source_code]
        src: String,

        #[label("This is type of '{a:?}'")]
        a_span: SourceSpan,
        a: Arc<Type>,

        ty: Type,
    },

    #[error("Type mismatch, '{a:?}' is not compatible with '{b:?}'")]
    Mismatch2 {
        #[source_code]
        src: String,

        #[label("This is type of '{a:?}'")]
        a_span: SourceSpan,
        a: Arc<Type>,

        #[label("This is type of '{b:?}'")]
        b_span: SourceSpan,
        b: Arc<Type>,
    },

    #[error("Cannot determine type from '{a:?}' and '{b:?}'")]
    Det2Failed {
        #[source_code]
        src: String,

        #[label("This is type of '{a:?}'")]
        a_span: SourceSpan,
        a: Type,

        #[label("This is type of '{b:?}'")]
        b_span: SourceSpan,
        b: Type,
    }
}

impl TypeError {
    pub fn new_cast_failed<'src, F, T>(from: &F, to: &T) -> miette::Report
    where
        F: Typed + Spanned<'src>,
        T: Typed,
    {
        TypeError::CastFailed {
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
        TypeError::Mismatch {
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
        TypeError::Mismatch2 {
            src: a.span().src.to_string(),
            a_span: a.span().into(),
            a: a.ty(),
            b_span: b.span().into(),
            b: b.ty(),
        }.into()
    }

    pub fn new_det2_failed<'src, A, B>(a: &A, b: &B) -> miette::Report
    where
        A: Typed + Spanned<'src>,
        B: Typed + Spanned<'src>,
    {
        TypeError::Det2Failed {
            src: a.span().src.to_string(),
            a_span: a.span().into(),
            a: a.ty().as_ref().clone(),
            b_span: b.span().into(),
            b: b.ty().as_ref().clone(),
        }.into()
    }
}
