use askama::Template;
use serde::{Deserialize, Serialize};

use crate::{client::{events::ClientEvent, htmx::HtmxSwap}, schema::endpoints::Endpoint};

#[derive(Template, Deserialize, Serialize, Default, Debug, PartialEq)]
#[template(path = "components/event_listener.html")]
pub struct EventListener {
    event: ClientEvent,
    target: String,
    endpoint: Endpoint,
    swap: HtmxSwap,
}

impl EventListener {
    pub fn render(event: ClientEvent, endpoint: Endpoint, target: &str, swap: HtmxSwap) -> Self {
        EventListener {
            event,
            endpoint,
            target: String::from(target),
            swap,
        }
    }
}
