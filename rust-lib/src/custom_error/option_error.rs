use std::{error, fmt};

#[derive(Debug, Clone)]
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

impl fmt::Display for OptionalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "None:{}", self.field)
    }
}

impl error::Error for OptionalError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[macro_export]
macro_rules! otor {
    ($x:expr) => {
        $x.ok_or(OptionalError::new(String::from("$x")))
    };
}
