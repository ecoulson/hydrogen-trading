use askama::Template;
use serde::{Deserialize, Serialize};

use crate::client::{events::ClientEvent, htmx::HtmxSwap};

pub struct EventListenerBuilder {
    event_listener: EventListener,
}

impl EventListenerBuilder {
    pub fn new() -> Self {
        Self {
            event_listener: EventListener::default(),
        }
    }

    pub fn build(self) -> EventListener {
        self.event_listener
    }

    pub fn event(mut self, event: ClientEvent) -> Self {
        self.event_listener.event = event;

        self
    }

    pub fn target(mut self, target: &str) -> Self {
        self.event_listener.target = String::from(target);

        self
    }

    pub fn endpoint(mut self, endpoint: &str) -> Self {
        self.event_listener.endpoint = String::from(endpoint);

        self
    }

    pub fn swap(mut self, swap: HtmxSwap) -> Self {
        self.event_listener.swap = swap;

        self
    }
}

#[derive(Template, Deserialize, Serialize, Default, Debug, PartialEq)]
#[template(path = "components/event_listener.html")]
pub struct EventListener {
    event: ClientEvent,
    target: String,
    endpoint: String,
    swap: HtmxSwap,
}
