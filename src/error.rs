use chrono::{DateTime, Utc, NaiveDateTime};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TimeParseError {
    #[error("invalid datetime input: {0}")]
    InvalidInput(String),

    #[error("ambiguous datetime (DST transition): {datetime} -> {options:?}")]
    AmbiguousTime {
        datetime: NaiveDateTime,
        options: Vec<DateTime<Utc>>,
    },
}
