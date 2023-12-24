use askama::Template;

use crate::{responders::htmx_responder::{HtmxTemplate, HtmxHeadersBuilder}, schema::{errors::Error, endpoints::Endpoint}};

use super::icon::{Icon, IconKind, IconSize, IconColor};

#[derive(Template, Default, Debug)]
#[template(path = "components/banner_error.html")]
pub struct BannerError {
    message: String,
    close_icon: Icon,
    endpoint: Endpoint
}

impl BannerError {
    pub fn new(message: &str) -> Self {
        Self {
            message: String::from(message),
            close_icon: Icon::render_filled(IconKind::Close, IconSize::Small, IconColor::Black),
            endpoint: Endpoint::CloseError
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
