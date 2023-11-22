use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Template, Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
#[template(path = "components/histogram.html")]
pub struct HistogramResponse {
    pub id: String,
    pub endpoint: String,
    pub chart: Histogram,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
pub struct Histogram {
    pub title: String,
    pub keys: Vec<String>,
    pub label: Labels,
    pub datasets: Vec<HistogramDataset>,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
pub struct Labels {
    pub x: String,
    pub y: String,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
pub struct HistogramDataset {
    pub label: String,
    pub data_points: Vec<f64>,
}
