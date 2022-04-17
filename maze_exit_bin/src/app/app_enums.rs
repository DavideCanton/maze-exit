use std::{env::VarError, error::Error, fmt::Display};

use enum_derive::ParseEnumError;

custom_derive! {
    #[derive(Debug, EnumFromStr)]
    pub enum UIType {
        Terminal,
        Gui,
    }
}

#[derive(Debug)]
pub enum UITypeError {
    ParseEnumError(ParseEnumError, String),
    VarError(VarError),
}

impl Display for UITypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UITypeError::ParseEnumError(_, s) => write!(f, "Could not parse the UI type: {}", s),
            UITypeError::VarError(e) => write!(f, "Error in getting env variable: {}", e),
        }
    }
}

impl Error for UITypeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            UITypeError::ParseEnumError(e, _) => Some(e),
            UITypeError::VarError(e) => Some(e),
        }
    }
}
