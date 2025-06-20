use thiserror::Error;

#[derive(Debug, Error)]
pub enum VarDeclError {
    #[error("Variable '{name}' is not declared in the current scope")]
    NotDeclared {
        name: String,
    },
}

impl VarDeclError {
    pub fn new_not_declared(name: String) -> anyhow::Error {
        VarDeclError::NotDeclared { name }.into()
    }
}
