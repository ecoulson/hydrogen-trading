use askama::Template;

use crate::schema::endpoints::Endpoint;

#[derive(Debug, Default)]
pub enum InputVariant {
    #[default]
    Default,
    Hidden,
}

#[derive(Template, Debug, Default)]
#[template(path = "components/input.html")]
pub struct Input {
    variant: InputVariant,
    value: String,
    name: String,
    placeholder: String,
    endpoint: Endpoint,
    trigger: String,
    target: String,
}

impl Input {
    pub fn render_hidden(value: &str, name: &str) -> Self {
        Self {
            value: String::from(value),
            name: String::from(name),
            variant: InputVariant::Hidden,
            placeholder: String::new(),
            endpoint: Endpoint::default(),
            trigger: String::new(),
            target: String::new(),
        }
    }

    pub fn render_search(name: &str, endpoint: Endpoint, placeholder: &str, target: &str) -> Self {
        Self {
            value: String::new(),
            name: String::from(name),
            variant: InputVariant::Default,
            placeholder: String::from(placeholder),
            endpoint,
            trigger: String::from("keyup changed delay:500ms"),
            target: String::from(target),
        }
    }
}
