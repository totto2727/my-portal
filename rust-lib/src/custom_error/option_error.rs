use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct OptionError {
    field: String,
}

impl OptionError {
    pub fn new<S: Into<String>>(field: S) -> OptionError {
        OptionError {
            field: field.into(),
        }
    }
}

impl fmt::Display for OptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "None:{}", self.field)
    }
}

impl error::Error for OptionError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[macro_export]
macro_rules! otor {
    ($x:expr) => {
        $x.ok_or(OptionError::new(String::from("$x")))
    };
}
