use std::fmt::Display;

pub trait ErrComposer<T> {
    fn compose(self) -> anyhow::Result<Vec<T>>;
}

impl<I, T, E> ErrComposer<T> for I
where
    I: Iterator<Item = Result<T, E>>,
    E: Display,
{
    fn compose(self) -> anyhow::Result<Vec<T>> {
        let mut results = Vec::new();
        let mut err_s = String::new();
        for item in self {
            match item {
                Ok(value) => results.push(value),
                Err(err) => err_s.push_str(&format!("{}\n", err)),
            }
        }

        if err_s.is_empty() {
            Ok(results)
        } else {
            Err(anyhow::anyhow!(err_s))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ErrComposer;

    #[test]
    fn test_ok() {
        let result  = vec![
            Ok::<i32, &str>(1),
            Ok::<i32, &str>(2),
            Ok::<i32, &str>(3),
        ].into_iter().compose();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn test_err() {
        let result = vec![
            Ok(1),
            Ok(2),
            Err("error1"),
            Ok(3),
            Err("error2")
        ].into_iter().compose();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "error1\nerror2\n");
    }
}
