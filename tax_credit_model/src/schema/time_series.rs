use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, PartialEq)]
pub struct TimeSeries {
    pub label: String,
    pub data_points: Vec<TimeSeriesEntry>
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq)]
pub struct TimeSeriesEntry {
    pub date: String,
    pub value: f32
}

#[derive(Template, Deserialize, Serialize, Default, Debug, PartialEq)]
#[template(path = "components/time_series_chart.html")]
pub struct TimeSeriesChart {
    pub title: String,
    pub id: String,
    pub time_series: TimeSeries
}
