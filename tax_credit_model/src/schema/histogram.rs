use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
pub struct Histogram {
    pub title: String,
    pub keys: Vec<String>,
    pub label: Labels,
    pub datasets: Vec<HistogramDataset>,
}

impl Histogram {
    pub fn render(
        title: &str,
        label: Labels,
        keys: Vec<&str>,
        datasets: Vec<HistogramDataset>,
    ) -> Self {
        Self {
            title: String::from(title),
            label,
            keys: keys.into_iter().map(|key| String::from(key)).collect(),
            datasets,
        }
    }
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
pub struct Labels {
    pub x: String,
    pub y: String,
}

impl Labels {
    pub fn render(x_label: &str, y_label: &str) -> Self {
        Self {
            x: String::from(x_label),
            y: String::from(y_label),
        }
    }
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
pub struct HistogramDataset {
    pub label: String,
    pub data_points: Vec<f64>,
}

impl HistogramDataset {
    pub fn render(label: &str, data_points: Vec<f64>) -> Self {
        Self {
            label: String::from(label),
            data_points,
        }
    }
}
