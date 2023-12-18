use askama::Template;
use serde::{Deserialize, Serialize};

use crate::client::{events::ClientEvent, htmx::HtmxSwap};

#[derive(Template, Deserialize, Serialize, Default, Debug, PartialEq)]
#[template(path = "components/event_listener.html")]
pub struct EventListener {
    event: ClientEvent,
    target: String,
    endpoint: String,
    swap: HtmxSwap,
}

impl EventListener {
    pub fn render(event: ClientEvent, endpoint: &str, target: &str, swap: HtmxSwap) -> Self {
        EventListener {
            event,
            endpoint: String::from(endpoint),
            target: String::from(target),
            swap,
        }
    }
}
