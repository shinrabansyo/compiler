use std::sync::Arc;

use thiserror::Error;
use miette::{Diagnostic, SourceSpan};

use sb_compiler_parse_cst::Spanned;

use crate::r#type::Type;
use crate::Typed;

#[derive(Debug, Error, Diagnostic)]
#[diagnostic(
    code(semantics::ty::infer),
    url("this link is not working yet"),
)]
pub enum TypeInferError {
    #[error("Cannot infer a type from '{from:?}'")]
    InferFailed {
        #[source_code]
        src: String,

        #[label("This is '{from:?}'")]
        from_span: SourceSpan,
        from: Arc<Type>,
    },
}

impl TypeInferError {
    pub fn new_infer_failed<'src, F>(from: &F) -> miette::Report
    where
        F: Typed + Spanned<'src>,
    {
        TypeInferError::InferFailed {
            src: from.span().src.to_string(),
            from_span: from.span().into(),
            from: from.ty(),
        }.into()
    }
}
