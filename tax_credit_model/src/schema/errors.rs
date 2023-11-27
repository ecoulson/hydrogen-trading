use askama::Template;
use rocket::http::Status;

use crate::{
    components::icon::{Icon, IconBuilder, IconColor, IconKind, IconSize},
    responders::htmx_responder::{HtmxHeadersBuilder, HtmxTemplate},
};

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

#[derive(Template, Default, Debug)]
#[template(path = "components/banner_error.html")]
pub struct BannerError {
    message: String,
    close_icon: Icon,
}

impl BannerError {
    pub fn new(message: &str) -> Self {
        Self {
            message: String::from(message),
            close_icon: IconBuilder::new()
                .fill(IconColor::Black)
                .kind(IconKind::Close)
                .size(IconSize::Small)
                .build(),
        }
    }

    pub fn to_htmx(self) -> HtmxTemplate<BannerError> {
        HtmxTemplate::new(
            HtmxHeadersBuilder::new()
                .reswap("afterbegin")
                .retarget("#banner-error")
                .build(),
            self,
        )
    }

    pub fn create_from_error(error: Error) -> HtmxTemplate<BannerError> {
        BannerError::create_from_message(&error.to_string())
    }

    pub fn create_from_message(message: &str) -> HtmxTemplate<BannerError> {
        BannerError::to_htmx(BannerError::new(message))
    }
}

impl From<Error> for BannerError {
    fn from(error: Error) -> Self {
        BannerError::new(&error.to_string())
    }
}

impl From<&str> for BannerError {
    fn from(value: &str) -> Self {
        BannerError::new(value)
    }
}

impl From<BannerError> for HtmxTemplate<BannerError> {
    fn from(value: BannerError) -> Self {
        value.to_htmx()
    }
}

impl From<Error> for HtmxTemplate<BannerError> {
    fn from(error: Error) -> Self {
        BannerError::create_from_error(error)
    }
}
