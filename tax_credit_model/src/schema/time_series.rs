use serde::{Deserialize, Serialize};

use super::{errors::Result, histogram::Labels, time::Timestamp};

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

impl TimeSeries {
    pub fn render<T, F>(label: &str, color: ChartColor, data: Vec<T>, operation: F) -> Result<Self>
    where
        F: Fn(&T) -> Result<TimeSeriesEntry>,
    {
        Ok(Self {
            label: String::from(label),
            color,
            data_points: data.iter().map(|element| operation(element)).collect::<Result<Vec<TimeSeriesEntry>>>()?
        })
    }
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
pub struct TimeSeriesEntry {
    pub date: String,
    pub color: ChartColor,
    pub value: f64,
}

impl TimeSeriesEntry {
    pub fn render(value: f64, date: &Timestamp, color: ChartColor) -> Result<Self> {
        Ok(Self {
            value,
            date: date.to_utc_date_time()?.to_rfc3339(),
            color,
        })
    }
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
pub struct TimeSeriesChart {
    pub title: String,
    pub labels: Labels,
    pub datasets: Vec<TimeSeries>,
}

impl TimeSeriesChart {
    pub fn render(title: &str, labels: Labels, datasets: Vec<TimeSeries>) -> Self {
        Self {
            title: String::from(title),
            labels,
            datasets,
        }
    }
}
