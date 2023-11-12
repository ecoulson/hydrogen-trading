use rocket::post;

use crate::{
    responders::htmx_responder::{HtmxHeadersBuilder, HtmxTemplate},
    schema::errors::BannerError,
};

#[post("/close_error")]
pub fn close_error_handler() -> HtmxTemplate<BannerError> {
    HtmxTemplate::new(
        HtmxHeadersBuilder::new().reswap("delete").build(),
        BannerError::new(""),
    )
}
