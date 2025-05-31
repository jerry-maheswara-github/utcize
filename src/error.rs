use chrono::{DateTime, NaiveDateTime, Utc};
use thiserror::Error;

/// Represents errors that can occur while parsing or normalizing datetime input.
#[derive(Error, Debug)]
pub enum TimeParseError {
    /// The input string could not be parsed into a valid datetime format.
    ///
    /// This can happen due to an invalid format, unrecognized timezone,
    /// or out-of-range values.
    #[error("invalid datetime input: {0}")]
    InvalidInput(String),

    /// The input represents an ambiguous local time (typically during a daylight saving transition),
    /// resulting in two possible valid UTC datetimes.
    ///
    /// This occurs when the same local time can map to multiple UTC times due to clock changes.
    #[error("ambiguous datetime (DST transition): {datetime} -> {options:?}")]
    AmbiguousTime {
        /// The ambiguous local datetime.
        datetime: NaiveDateTime,

        /// The two possible UTC interpretations.
        options: Vec<DateTime<Utc>>,
    },
}
