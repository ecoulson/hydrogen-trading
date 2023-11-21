use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Template, Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
#[template(path = "components/histogram.html")]
pub struct Histogram {
    pub title: String,
    pub id: String,
    pub data_set_endpoints: Vec<String>,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq)]
pub struct HistogramData {
    pub labels: Vec<String>,
    pub datasets: Vec<HistogramDataset>
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq)]
pub struct HistogramDataset {
    pub label: String,
    pub data_points: Vec<f64>,
}
