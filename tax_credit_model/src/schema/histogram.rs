use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Template, Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
#[template(path = "components/histogram.html")]
pub struct Histogram {
    pub title: String,
    pub id: String,
    pub histogram_end_point: String,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq)]
pub struct HistogramData {
    pub labels: Labels,
    pub keys: Vec<String>,
    pub datasets: Vec<HistogramDataset>,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq)]
pub struct Labels {
    pub x: String,
    pub y: String,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq)]
pub struct HistogramDataset {
    pub label: String,
    pub data_points: Vec<f64>,
}
