use std::io::Cursor;

use askama;
use rocket::{http::ContentType, response::Responder, Response};

pub const HX_TRIGGER: &str = "HX-Trigger";
pub const HX_PUSH_URL: &str = "HX-Push-Url";
pub const HX_LOCATION: &str = "HX-Location";
pub const HX_RESWAP: &str = "HX-Reswap";
pub const HX_REFRESH: &str = "HX-Refresh";
pub const HX_REDIRECT: &str = "HX-Redirect";
pub const HX_RETARGET: &str = "HX-Retarget";
pub const HX_RESELECT: &str = "HX-Reselect";
pub const HX_REPLACE_URL: &str = "HX-Replace-Url";
pub const HX_TRIGGER_AFTER_SWAP: &str = "HX-Trigger-After-Swap";
pub const HX_TRIGGER_AFTER_SETTLE: &str = "HX-Trigger-After-Settle";

pub struct HtmxTemplate<T>
where
    T: askama::Template,
{
    html: T,
    headers: HtmxHeaders,
}

#[derive(Default)]
pub struct HtmxHeaders {
    pub location: Option<String>,
    pub push_url: Option<String>,
    pub redirect: Option<String>,
    pub refresh: Option<String>,
    pub replace_url: Option<String>,
    pub reswap: Option<String>,
    pub retarget: Option<String>,
    pub reselect: Option<String>,
    pub trigger: Option<String>,
    pub trigger_after_settle: Option<String>,
    pub trigger_after_swap: Option<String>,
}

impl HtmxHeaders {
    pub fn set_header(&mut self, name: &str, value: &str) {
        match name {
            HX_TRIGGER => self.trigger = Some(String::from(value)),
            HX_LOCATION => self.location = Some(String::from(value)),
            HX_PUSH_URL => self.push_url = Some(String::from(value)),
            HX_REDIRECT => self.redirect = Some(String::from(value)),
            HX_REFRESH => self.refresh = Some(String::from(value)),
            HX_REPLACE_URL => self.replace_url = Some(String::from(value)),
            HX_RESWAP => self.reswap = Some(String::from(value)),
            HX_RETARGET => self.retarget = Some(String::from(value)),
            HX_RESELECT => self.reselect = Some(String::from(value)),
            HX_TRIGGER_AFTER_SETTLE => self.trigger_after_settle = Some(String::from(value)),
            HX_TRIGGER_AFTER_SWAP => self.trigger_after_swap = Some(String::from(value)),
            _ => return,
        }
    }
}

impl<T> HtmxTemplate<T>
where
    T: askama::Template,
{
    pub fn new(template: T, headers: HtmxHeaders) -> HtmxTemplate<T> {
        HtmxTemplate {
            html: template,
            headers,
        }
    }
}

impl<T> From<T> for HtmxTemplate<T>
where
    T: askama::Template,
{
    fn from(value: T) -> Self {
        HtmxTemplate::<T>::new(value, HtmxHeaders::default())
    }
}

impl<'r, T> Responder<'r, 'static> for HtmxTemplate<T>
where
    T: askama::Template,
{
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let template = self.html.to_string();
        let mut response = Response::build()
            .header(ContentType::HTML)
            .sized_body(template.len(), Cursor::new(template))
            .ok()?;

        if let Some(trigger) = self.headers.trigger {
            response.set_raw_header(HX_TRIGGER, trigger);
        }
        if let Some(push_url) = self.headers.push_url {
            response.set_raw_header(HX_LOCATION, push_url);
        }
        if let Some(location) = self.headers.location {
            response.set_raw_header(HX_LOCATION, location);
        }
        if let Some(reswap) = self.headers.reswap {
            response.set_raw_header(HX_RESWAP, reswap);
        }
        if let Some(refresh) = self.headers.refresh {
            response.set_raw_header(HX_REFRESH, refresh);
        }
        if let Some(redirect) = self.headers.redirect {
            response.set_raw_header(HX_REDIRECT, redirect);
        }
        if let Some(retarget) = self.headers.retarget {
            response.set_raw_header(HX_RETARGET, retarget);
        }
        if let Some(reselect) = self.headers.reselect {
            response.set_raw_header(HX_RESELECT, reselect);
        }
        if let Some(replace_url) = self.headers.replace_url {
            response.set_raw_header(HX_REPLACE_URL, replace_url);
        }
        if let Some(trigger_after_swap) = self.headers.trigger_after_swap {
            response.set_raw_header(HX_TRIGGER_AFTER_SWAP, trigger_after_swap);
        }
        if let Some(trigger_after_settle) = self.headers.trigger_after_settle {
            response.set_raw_header(HX_TRIGGER_AFTER_SETTLE, trigger_after_settle);
        }

        Ok(response)
    }
}
