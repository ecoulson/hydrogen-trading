use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub enum BadgeVariant {
    #[default]
    Default,
    Info,
}

#[derive(Template, Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
#[template(path = "components/badge.html")]
pub struct Badge {
    pub variant: BadgeVariant,
    pub text: String,
}

impl Badge {
    pub fn render(text: &str) -> Badge {
        Badge {
            variant: BadgeVariant::Default,
            text: String::from(text),
        }
    }

    pub fn render_info(text: &str) -> Badge {
        Badge {
            variant: BadgeVariant::Info,
            text: String::from(text),
        }
    }
}
