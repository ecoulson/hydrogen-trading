use std::collections::HashMap;

use askama::Template;
use nanoid::nanoid;

use crate::schema::{endpoints::Endpoint, time_series::TimeSeriesChart};

use super::input::Input;

#[derive(Template, Default, Debug)]
#[template(path = "components/time_series_chart.html")]
pub struct TimeSeriesChartResponse {
    pub id: String,
    pub endpoint: Input,
    pub chart: TimeSeriesChart,
    pub parameters: Vec<Input>,
}

impl TimeSeriesChartResponse {
    pub fn render(
        chart: TimeSeriesChart,
        endpoint: Endpoint,
        parameters: HashMap<&str, String>,
    ) -> Self {
        Self {
            id: nanoid!(),
            endpoint: Input::render_hidden(&endpoint.to_string(), "endpoint"),
            parameters: parameters
                .iter()
                .map(|(key, value)| Input::render_hidden(key, value))
                .collect(),
            chart,
        }
    }
}
