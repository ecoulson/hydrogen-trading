use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, PartialEq)]
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

#[derive(Deserialize, Serialize, Default, Debug, PartialEq)]
pub struct TimeSeries {
    pub color: ChartColor,
    pub label: String,
    pub data_points: Vec<TimeSeriesEntry>,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq)]
pub struct TimeSeriesEntry {
    pub date: String,
    pub color: ChartColor,
    pub value: f64,
}

#[derive(Template, Deserialize, Serialize, Default, Debug, PartialEq)]
#[template(path = "components/time_series_chart.html")]
pub struct TimeSeriesChart {
    pub title: String,
    pub id: String,
    pub time_series: Vec<TimeSeries>,
}
