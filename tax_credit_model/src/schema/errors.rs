use rocket::http::Status;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    NotFound(String),
    Poisoned(String),
    Unimplemented(String),
    InvalidArgument(String),
    Unauthenticated(String),
    Unknown(String),
}

impl From<Error> for Status {
    fn from(value: Error) -> Self {
        match value {
            Error::NotFound(_) => Status::NotFound,
            Error::Unimplemented(_) => Status::NotImplemented,
            Error::InvalidArgument(_) => Status::BadRequest,
            Error::Unauthenticated(_) => Status::Unauthorized,
            Error::Unknown(_) | Error::Poisoned(_) => Status::InternalServerError,
        }
    }
}

// TODO: have these functions accept an error instead of a string
impl Error {
    pub fn not_found(value: &str) -> Error {
        Error::NotFound(String::from(value))
    }

    pub fn poison(value: &str) -> Error {
        Error::Poisoned(String::from(value))
    }

    pub fn unimplemented(value: &str) -> Error {
        Error::Unimplemented(String::from(value))
    }

    pub fn invalid_argument(value: &str) -> Error {
        Error::InvalidArgument(String::from(value))
    }

    pub fn unknown(value: &str) -> Error {
        Error::Unknown(String::from(value))
    }

    pub fn unauthenticated(value: &str) -> Error {
        Error::Unauthenticated(String::from(value))
    }
}

// TODO: using errors convert to string to make easier
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::NotFound(value) => write!(f, "NotFound: {}", value),
            Self::Poisoned(value) => write!(f, "Poisoned: {}", value),
            Self::Unimplemented(value) => write!(f, "Unimplemented: {}", value),
            Self::InvalidArgument(value) => write!(f, "Invalid Argument: {}", value),
            Self::Unauthenticated(value) => write!(f, "Unauthenticated: {}", value),
            Self::Unknown(value) => write!(f, "Unknown: {}", value),
        }
    }
}
