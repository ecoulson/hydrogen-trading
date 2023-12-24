use rocket::post;

use crate::{
    components::error::BannerError,
    responders::htmx_responder::{HtmxHeadersBuilder, HtmxTemplate},
};

#[post("/close_error")]
pub fn close_error_handler() -> HtmxTemplate<BannerError> {
    HtmxTemplate::new(
        HtmxHeadersBuilder::new().reswap("delete").build(),
        BannerError::new(""),
    )
}
