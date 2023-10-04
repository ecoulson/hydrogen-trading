use askama::Template;
use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    Parse(String),
    NotFound(String),
    Poisoned(String),
    Unimplemented(String),
    InvalidArgument(String),
    Unknown(String),
}

// TODO: have these functions accept an error instead of a string
impl Error {
    pub fn create_parse_error(value: &str) -> Error {
        Error::Parse(String::from(value))
    }

    pub fn create_not_found_error(value: &str) -> Error {
        Error::NotFound(String::from(value))
    }

    pub fn create_poison_error(value: &str) -> Error {
        Error::Poisoned(String::from(value))
    }

    pub fn create_unimplemented_error(value: &str) -> Error {
        Error::Unimplemented(String::from(value))
    }

    pub fn create_invalid_argument_error(value: &str) -> Error {
        Error::InvalidArgument(String::from(value))
    }

    pub fn create_unknown_error(value: &str) -> Error {
        Error::Unknown(String::from(value))
    }
}

// TODO: using errors convert to string to make easier
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Parse(value) => write!(f, "Parse: {}", value),
            Self::NotFound(value) => write!(f, "NotFound: {}", value),
            Self::Poisoned(value) => write!(f, "Poisoned: {}", value),
            Self::Unimplemented(value) => write!(f, "Unimplemented: {}", value),
            Self::InvalidArgument(value) => write!(f, "Invalid Argument: {}", value),
            Self::Unknown(value) => write!(f, "Unknown: {}", value),
        }
    }
}

#[derive(Template, Deserialize, Serialize, Default, Debug, PartialEq)]
#[template(path = "components/banner_error.html")]
pub struct BannerError {
    pub message: String,
}
