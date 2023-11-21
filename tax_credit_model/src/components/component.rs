use askama::Template;

use crate::responders::htmx_responder::{HtmxHeaders, HtmxTemplate};

pub type ComponentResponse<T, E> = Result<HtmxTemplate<T>, HtmxTemplate<E>>;

pub struct Component;

impl Component {
    pub fn component<T, E>(headers: HtmxHeaders, value: T) -> ComponentResponse<T, E>
    where
        T: Template,
        E: Template,
    {
        Ok(HtmxTemplate::new(headers, value))
    }

    pub fn basic<T, E>(value: T) -> ComponentResponse<T, E>
    where
        T: Template,
        E: Template,
    {
        Ok(HtmxTemplate::template(value))
    }
}
