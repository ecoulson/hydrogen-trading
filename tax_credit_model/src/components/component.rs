use askama::Template;

use crate::{
    responders::htmx_responder::{HtmxHeaders, HtmxTemplate},
    schema::errors::{BannerError, Error},
};

pub type ComponentResponse<T, E> = Result<HtmxTemplate<T>, HtmxTemplate<E>>;

pub struct Component;

impl Component {
    pub fn new<T, E>(headers: HtmxHeaders, value: T) -> ComponentResponse<T, E>
    where
        T: Template,
        E: Template,
    {
        Ok(HtmxTemplate::new(headers, value))
    }

    pub fn htmx<T, E>(value: T) -> ComponentResponse<T, E>
    where
        T: Template,
        E: Template,
    {
        Ok(HtmxTemplate::template(value))
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
