use askama::Template;
use rocket::http::Status;

use crate::responders::htmx_responder::{HtmxHeaders, HtmxTemplate};

pub type PageResponse<T> = Result<HtmxTemplate<T>, Status>;

pub struct Page;

impl Page {
    pub fn page<T>(headers: HtmxHeaders, value: T) -> PageResponse<T>
    where
        T: Template,
    {
        Ok(HtmxTemplate::new(headers, value))
    }

    pub fn basic_page<T>(value: T) -> PageResponse<T>
    where
        T: Template,
    {
        Ok(HtmxTemplate::template(value))
    }
}
