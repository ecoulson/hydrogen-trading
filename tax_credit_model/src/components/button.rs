use std::fmt::Display;

use askama::Template;
use serde::{Deserialize, Serialize};

use crate::client::htmx::HtmxSwap;

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Outline,
    Disabled,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum ButtonState {
    #[default]
    Enabled,
    Disabled,
}

impl Display for ButtonState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Enabled => write!(f, ""),
            Self::Disabled => write!(f, "disabled"),
        }
    }
}

#[derive(Template, Debug, Default, PartialEq, Serialize, Deserialize)]
#[template(path = "components/button.html")]
pub struct Button {
    state: ButtonState,
    text: String,
    swap: HtmxSwap,
    target: String,
    endpoint: String,
    variant: ButtonVariant,
}

impl Button {
    pub fn disable(&mut self) {
        self.state = ButtonState::Disabled;
        self.variant = ButtonVariant::Disabled;
    }
}

#[derive(Debug)]
pub struct ButtonBuilder {
    button: Button,
}

impl ButtonBuilder {
    pub fn new() -> Self {
        Self {
            button: Button::default(),
        }
    }

    pub fn swap(mut self, swap: HtmxSwap) -> Self {
        self.button.swap = swap;

        self
    }

    pub fn target(mut self, target: &str) -> Self {
        self.button.target = String::from(target);

        self
    }

    pub fn endpoint(mut self, endpoint: &str) -> Self {
        self.button.endpoint = String::from(endpoint);

        self
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.button.variant = variant;

        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.button.text = String::from(text);

        self
    }

    pub fn set_disabled(mut self, disabled: bool) -> Self {
        if disabled {
            self.button.disable();
        }

        self
    }

    pub fn disabled(mut self) -> Self {
        self.button.disable();

        self
    }

    pub fn build(self) -> Button {
        self.button
    }
}
