use askama::Template;
use serde::{Deserialize, Serialize};

use crate::components::input::Input;

use super::histogram::Labels;

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
pub enum ChartColor {
    #[default]
    Red,
    Green,
    Chartreuse,
    Yellow,
    Orange,
    Blue,
}

impl std::fmt::Display for ChartColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blue => write!(f, "blue"),
            Self::Red => write!(f, "red"),
            Self::Green => write!(f, "green"),
            Self::Yellow => write!(f, "yellow"),
            Self::Orange => write!(f, "orange"),
            Self::Chartreuse => write!(f, "#7fff00"),
        }
    }
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
pub struct TimeSeries {
    pub color: ChartColor,
    pub label: String,
    pub data_points: Vec<TimeSeriesEntry>,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
pub struct TimeSeriesEntry {
    pub date: String,
    pub color: ChartColor,
    pub value: f64,
}

#[derive(Template, Default, Debug)]
#[template(path = "components/time_series_chart.html")]
pub struct TimeSeriesChartResponse {
    pub id: String,
    pub endpoint_input: Input,
    pub chart: TimeSeriesChart,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
pub struct TimeSeriesChart {
    pub title: String,
    pub labels: Labels,
    pub datasets: Vec<TimeSeries>,
}
