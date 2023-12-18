use askama::Template;

#[derive(Debug, Default, Template)]
#[template(path = "components/select.html")]
pub struct Select {
    name: String,
    default: String,
    options: Vec<String>,
}

#[derive(Debug, Default)]
pub struct SelectBuilder {
    select: Select,
}

impl SelectBuilder {
    pub fn new() -> Self {
        Self {
            select: Select::default(),
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.select.name = String::from(name);

        self
    }

    pub fn default(mut self, default: &str) -> Self {
        self.select.default = String::from(default);

        self
    }

    pub fn options(mut self, options: Vec<String>) -> Self {
        self.select.options = options;

        self
    }

    pub fn build(self) -> Select {
        self.select
    }
}
