use askama::Template;

#[derive(Debug, Default, Template)]
#[template(path = "components/select.html")]
pub struct Select {
    name: String,
    default: String,
    options: Vec<String>,
}

impl Select {
    pub fn render(name: &str, default: &str, options: Vec<String>) -> Self {
        Select {
            name: String::from(name),
            default: String::from(default),
            options,
        }
    }
}
