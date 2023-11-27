use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub enum BadgeVariant {
    #[default]
    Primary,
    Secondary,
}

#[derive(Template, Debug, PartialEq, Serialize, Deserialize, Default)]
#[template(path = "components/badge.html")]
pub struct Badge {
    variant: BadgeVariant,
    text: String,
}

pub struct BadgeBuilder {
    badge: Badge,
}

impl BadgeBuilder {
    pub fn new() -> Self {
        Self {
            badge: Badge::default(),
        }
    }

    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.badge.variant = variant;

        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.badge.text = String::from(text);

        self
    }

    pub fn build(self) -> Badge {
        self.badge
    }
}
