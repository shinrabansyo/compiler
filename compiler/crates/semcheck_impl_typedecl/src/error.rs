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
    pub fn new_already_declared(name: String) -> miette::Report {
        miette::miette!("Type {} is already declared.", name)
        // TypeDeclError::TypeAlreadyDeclared { name }.into()
    }

    pub fn new_not_declared(name: String) -> miette::Report {
        miette::miette!("Type {} is not declared.", name)
        // TypeDeclError::TypeNotDeclared { name }.into()
    }

    pub fn new_not_declared_in_scope(name: String) -> miette::Report {
        miette::miette!("Type {} is not declared in this scope.", name)
        // TypeDeclError::TypeNotDeclaredInScope { name }.into()
    }
}
