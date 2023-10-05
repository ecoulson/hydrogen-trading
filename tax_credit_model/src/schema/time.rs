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
    pub fn new(seconds: i64, nanos: u32) -> Timestamp {
        Timestamp { seconds, nanos }
    }

    pub fn to_utc_date_time(&self) -> Result<DateTime<Utc>> {
        Utc.timestamp_opt(self.seconds, self.nanos)
            .single()
            .ok_or_else(|| Error::create_parse_error("Invalid timestamp"))
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

#[derive(FromForm, Deserialize, Serialize, Default, Debug, PartialEq, Eq)]
pub struct DateTimeRange {
    pub start: String,
    pub end: String,
}

impl DateTimeRange {
    pub fn parse(&self, format: &str) -> Result<TimeRange> {
        let start = NaiveDateTime::parse_from_str(&self.start, format)
            .map_err(|err| Error::create_invalid_argument_error(&err.to_string()))?;
        let end = NaiveDateTime::parse_from_str(&self.end, format)
            .map_err(|err| Error::create_invalid_argument_error(&err.to_string()))?;

        Ok(TimeRange {
            start: Timestamp::new(start.timestamp(), start.timestamp_subsec_nanos()),
            end: Timestamp::new(end.timestamp(), end.timestamp_subsec_nanos()),
        })
    }
}
