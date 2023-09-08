use chrono::{DateTime, LocalResult, TimeZone, Utc};
use rocket::FromForm;
use serde::{Deserialize, Serialize};

#[derive(FromForm, Deserialize, Serialize, Default, Debug, PartialEq, Eq)]
pub struct Timestamp {
    pub seconds: i64,
    pub nanos: u32,
}

impl Timestamp {
    pub fn new(seconds: i64, nanos: u32) -> Timestamp {
        Timestamp { seconds, nanos }
    }

    pub fn to_utc_date_time(&self) -> LocalResult<DateTime<Utc>> {
        Utc.timestamp_opt(self.seconds, self.nanos)
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

#[derive(FromForm, Deserialize, Serialize, Default, Debug, PartialEq, Eq)]
pub struct TimeRange {
    pub start: Timestamp,
    pub end: Timestamp,
}
