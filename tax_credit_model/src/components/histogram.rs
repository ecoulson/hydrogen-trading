use std::collections::HashMap;

use askama::Template;
use nanoid::nanoid;

use crate::schema::{endpoints::Endpoint, histogram::Histogram};

use super::input::Input;

#[derive(Template, Default, Debug)]
#[template(path = "components/histogram.html")]
pub struct HistogramResponse {
    pub id: String,
    pub endpoint: Input,
    pub chart: Histogram,
    pub parameters: Vec<Input>,
}

impl HistogramResponse {
    pub fn render(endpoint: Endpoint, parameters: HashMap<&str, String>, chart: Histogram) -> Self {
        Self {
            id: nanoid!(),
            chart,
            endpoint: Input::render_hidden(&endpoint.to_string(), "endpoint"),
            parameters: parameters
                .iter()
                .map(|(key, value)| Input::render_hidden(value, key))
                .collect(),
        }
    }
}
