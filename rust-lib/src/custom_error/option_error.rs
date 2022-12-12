use thiserror::Error;

#[derive(Debug, Error)]
#[error("None:{field}")]
pub struct OptionalError {
    field: String,
}

impl OptionalError {
    pub fn new<S: Into<String>>(field: S) -> OptionalError {
        OptionalError {
            field: field.into(),
        }
    }
}

mod macros {
    #[macro_export]
    macro_rules! otor {
        ($x:expr) => {
            $x.ok_or(rust_lib::custom_error::OptionalError::new("$x".to_owned()))
        };
    }
}
