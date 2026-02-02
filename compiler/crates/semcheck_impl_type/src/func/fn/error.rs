use thiserror::Error;
use miette::{Diagnostic, SourceSpan};

use sb_compiler_parse_cst::{Span, Spanned};

use crate::Typed;

#[derive(Debug, Error, Diagnostic)]
#[diagnostic(
    code(semantics::ty::r#fn),
    url("this link is not working yet"),
)]
pub enum TypeFnError {
    #[error("Function requires {} arguments, but got {}.", expected, got)]
    ArgumentNumsMisMatch {
        #[source_code]
        src: String,

        #[label("here")]
        span: SourceSpan,

        expected: usize,
        got: usize,
    },

    #[error("Function requires a return type matching '{requires}', but got '{ret}'")]
    ReturnFailed {
        #[source_code]
        src: String,

        #[label("This is '{ret}'")]
        ret_span: SourceSpan,
        ret: String,

        requires: String,
    },
}

impl TypeFnError {
    pub fn new_argument_nums_mis_match(span: Span, expected: usize, got: usize) -> miette::Report {
        TypeFnError::ArgumentNumsMisMatch {
            src: span.src.to_string(),
            span: span.into(),
            expected,
            got,
        }.into()
    }

    pub fn new_return_failed<'src, Rq, Rt>(requires: &Rq, ret: &Rt) -> miette::Report
    where
        Rq: Typed,
        Rt: Typed + Spanned<'src>,
    {
        TypeFnError::ReturnFailed {
            src: ret.span().src.to_string(),
            ret_span: ret.span().into(),
            ret: ret.ty().to_string(),
            requires: requires.ty().to_string(),
        }.into()
    }
}
