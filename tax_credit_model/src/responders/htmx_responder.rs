use std::io::Cursor;

use askama;
use rocket::{http::ContentType, response::Responder, Response};

use crate::{client::events::ClientEvent, schema::errors::Error};

const HX_TRIGGER: &str = "HX-Trigger";
const HX_PUSH_URL: &str = "HX-Push-Url";
const HX_LOCATION: &str = "HX-Location";
const HX_RESWAP: &str = "HX-Reswap";
const HX_REFRESH: &str = "HX-Refresh";
const HX_REDIRECT: &str = "HX-Redirect";
const HX_RETARGET: &str = "HX-Retarget";
const HX_RESELECT: &str = "HX-Reselect";
const HX_REPLACE_URL: &str = "HX-Replace-Url";
const HX_TRIGGER_AFTER_SWAP: &str = "HX-Trigger-After-Swap";
const HX_TRIGGER_AFTER_SETTLE: &str = "HX-Trigger-After-Settle";
const SET_COOKIE: &str = "Set-Cookie";

pub struct HtmxTemplate<T>
where
    T: askama::Template,
{
    html: T,
    headers: HtmxHeaders,
}

#[derive(Default, Debug)]
pub struct HtmxHeaders {
    location: Option<String>,
    push_url: Option<String>,
    redirect: Option<String>,
    refresh: Option<String>,
    replace_url: Option<String>,
    reswap: Option<String>,
    retarget: Option<String>,
    reselect: Option<String>,
    trigger: Option<String>,
    trigger_after_settle: Option<String>,
    trigger_after_swap: Option<String>,
    set_cookie: Option<String>,
}

#[derive(Default, Debug)]
pub struct HtmxHeadersBuilder {
    headers: HtmxHeaders,
}

impl HtmxHeadersBuilder {
    pub fn new() -> Self {
        Self {
            headers: HtmxHeaders::default(),
        }
    }

    pub fn location(mut self, value: &str) -> Self {
        self.headers.location = Some(String::from(value));

        self
    }

    pub fn push_url(mut self, value: &str) -> Self {
        self.headers.push_url = Some(String::from(value));

        self
    }

    pub fn redirect(mut self, value: &str) -> Self {
        self.headers.redirect = Some(String::from(value));

        self
    }

    pub fn refresh(mut self, value: &str) -> Self {
        self.headers.refresh = Some(String::from(value));

        self
    }

    pub fn replace_url(mut self, value: &str) -> Self {
        self.headers.replace_url = Some(String::from(value));

        self
    }

    pub fn reswap(mut self, value: &str) -> Self {
        self.headers.reswap = Some(String::from(value));

        self
    }

    pub fn retarget(mut self, value: &str) -> Self {
        self.headers.retarget = Some(String::from(value));

        self
    }

    pub fn reselect(mut self, value: &str) -> Self {
        self.headers.reselect = Some(String::from(value));

        self
    }

    pub fn trigger(mut self, value: ClientEvent) -> Self {
        self.headers.trigger = Some(value.to_string());

        self
    }

    pub fn trigger_after_settle(mut self, value: &str) -> Self {
        self.headers.trigger_after_settle = Some(String::from(value));

        self
    }

    pub fn trigger_after_swap(mut self, value: &str) -> Self {
        self.headers.trigger_after_swap = Some(String::from(value));

        self
    }

    pub fn set_cookie(mut self, value: &str) -> Self {
        self.headers.set_cookie = Some(String::from(value));

        self
    }

    pub fn set_cookie_if(mut self, value: Option<String>) -> Self {
        if value.is_some() {
            self.headers.set_cookie = value;
        }

        self
    }

    pub fn build(self) -> HtmxHeaders {
        self.headers
    }
}

impl<T> HtmxTemplate<T>
where
    T: askama::Template,
{
    pub fn new(headers: HtmxHeaders, template: T) -> Self {
        Self {
            html: template,
            headers,
        }
    }

    pub fn template(template: T) -> Self {
        Self {
            html: template,
            headers: HtmxHeaders::default(),
        }
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
            .ok::<Error>()?;

        if let Some(trigger) = self.headers.trigger {
            response.set_raw_header(HX_TRIGGER, trigger);
        }
        if let Some(push_url) = self.headers.push_url {
            response.set_raw_header(HX_PUSH_URL, push_url);
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
        if let Some(cookie) = self.headers.set_cookie {
            response.set_raw_header(SET_COOKIE, cookie);
        }

        Ok(response)
    }
}
