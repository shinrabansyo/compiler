use miette::{Diagnostic, Report};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("{msg}")]
pub struct ComposedError {
    #[related]
    errs: Vec<Report>,
    msg: String,
}

pub trait ErrComposer<T> {
    fn compose(self) -> miette::Result<Vec<T>>;
    fn compose_with(self, msg: &str) -> miette::Result<Vec<T>>;
}

impl<I, T> ErrComposer<T> for I
where
    I: Iterator<Item = Result<T, Report>>,
{
    fn compose(self) -> miette::Result<Vec<T>> {
        self.compose_with("Some errors occurred")
    }

    fn compose_with(self, msg: &str) -> miette::Result<Vec<T>> {
        let mut results = vec![];
        let mut errs = vec![];
        for item in self {
            match item {
                Ok(value) => results.push(value),
                Err(err) => errs.push(err),
            }
        }

        if errs.is_empty() {
            Ok(results)
        } else {
            Err(ComposedError { errs, msg: msg.into(), }.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use miette::Report;

    use super::ErrComposer;

    #[test]
    fn test_ok() {
        let result  = vec![
            Ok::<i32, Report>(1),
            Ok::<i32, Report>(2),
            Ok::<i32, Report>(3),
        ].into_iter().compose();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn test_err() {
        let result = vec![
            Ok(1),
            Ok(2),
            Err(miette::miette!("error1")),
            Ok(3),
            Err(miette::miette!("error2")),
        ].into_iter().compose();

        assert!(result.is_err());
    }
}
