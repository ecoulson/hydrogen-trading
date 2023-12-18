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

#[derive(Template, Debug, Default, PartialEq, Serialize, Deserialize)]
#[template(path = "components/button.html")]
pub struct Button {
    pub text: String,
    pub swap: HtmxSwap,
    pub target: String,
    pub endpoint: String,
    pub variant: ButtonVariant,
}

impl Button {
    pub fn render(text: &str, endpoint: &str, target: &str) -> Self {
        let mut button = Button::default();
        button.text = String::from(text);
        button.endpoint = String::from(endpoint);
        button.target = String::from(target);

        button
    }

    pub fn render_outline(text: &str, endpoint: &str, target: &str) -> Self {
        let mut button = Button::render(text, endpoint, target);
        button.variant = ButtonVariant::Outline;

        button
    }

    pub fn render_secondary(text: &str, endpoint: &str, target: &str) -> Self {
        let mut button = Button::render(text, endpoint, target);
        button.variant = ButtonVariant::Secondary;

        button
    }
}
