use std::cmp::Ordering;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use rocket::FromForm;
use serde::{Deserialize, Serialize};

use super::errors::{Error, Result};

#[derive(FromForm, Deserialize, Serialize, Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Timestamp {
    pub seconds: i64,
    pub nanos: u32,
}

impl Timestamp {
    pub fn new(seconds: i64, nanos: u32) -> Self {
        Self { seconds, nanos }
    }

    pub fn to_utc_date_time(&self) -> Result<DateTime<Utc>> {
        Utc.timestamp_opt(self.seconds, self.nanos)
            .single()
            .ok_or_else(|| Error::invalid_argument("Invalid timestamp"))
    }
}

impl Ord for Timestamp {
    fn cmp(&self, other: &Self) -> Ordering {
        let seconds_order = self.seconds.cmp(&other.seconds);
        let nanos_order = self.nanos.cmp(&other.nanos);

        match seconds_order {
            Ordering::Equal => nanos_order,
            _ => seconds_order,
        }
    }
}

impl PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> From<DateTime<T>> for Timestamp
where
    T: TimeZone,
{
    fn from(value: DateTime<T>) -> Self {
        Timestamp {
            seconds: value.timestamp(),
            nanos: value.timestamp_subsec_nanos(),
        }
    }
}

impl From<NaiveDateTime> for Timestamp {
    fn from(value: NaiveDateTime) -> Self {
        Timestamp {
            seconds: value.timestamp(),
            nanos: value.timestamp_subsec_nanos(),
        }
    }
}

#[derive(FromForm, Deserialize, Serialize, Default, Debug, PartialEq, Eq)]
pub struct TimeRange {
    pub start: Timestamp,
    pub end: Timestamp,
}

impl TimeRange {
    pub fn to_datetime(&self) -> Result<DateTimeRange> {
        Ok(DateTimeRange {
            start: self.start.to_utc_date_time()?.to_rfc3339(),
            end: self.end.to_utc_date_time()?.to_rfc3339(),
        })
    }
}

#[derive(FromForm, Deserialize, Serialize, Default, Debug, PartialEq, Eq)]
pub struct DateTimeRange {
    pub start: String,
    pub end: String,
}

impl DateTimeRange {
    pub fn parse(&self, format: &str) -> Result<TimeRange> {
        let start = NaiveDateTime::parse_from_str(&self.start, format)
            .map_err(|err| Error::invalid_argument(&err.to_string()))?;
        let end = NaiveDateTime::parse_from_str(&self.end, format)
            .map_err(|err| Error::invalid_argument(&err.to_string()))?;

        Ok(TimeRange {
            start: Timestamp::new(start.timestamp(), start.timestamp_subsec_nanos()),
            end: Timestamp::new(end.timestamp(), end.timestamp_subsec_nanos()),
        })
    }
}
