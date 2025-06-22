use thiserror::Error;

#[derive(Debug, Error)]
pub enum TypeDeclError {
    #[error("Type {name} is already declared.")]
    TypeAlreadyDeclared {
        name: String,
    },
    #[error("Type {name} is not declared.")]
    TypeNotDeclared {
        name: String,
    },
    #[error("Type {name} is not declared in this scope.")]
    TypeNotDeclaredInScope {
        name: String,
    },
}

impl TypeDeclError {
    pub fn new_already_declared(name: String) -> Self {
        TypeDeclError::TypeAlreadyDeclared { name }
    }

    pub fn new_not_declared(name: String) -> Self {
        TypeDeclError::TypeNotDeclared { name }
    }

    pub fn new_not_declared_in_scope(name: String) -> Self {
        TypeDeclError::TypeNotDeclaredInScope { name }
    }
}
