use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use chrono_tz::Tz;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum EpochKind {
    Seconds,
    Milliseconds,
    Microseconds,
    Nanoseconds,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum TimeInput {
    Iso(DateTime<Utc>),
    Epoch(EpochKind, DateTime<Utc>),
    Invalid(String),
}

impl TimeInput {
    pub fn as_datetime(&self) -> Option<DateTime<Utc>> {
        match self {
            TimeInput::Iso(dt) => Some(*dt),
            TimeInput::Epoch(_, dt) => Some(*dt),
            TimeInput::Invalid(_) => None,
        }
    }
}


#[derive(Debug)]
pub enum TimeZoneParsed {
    FixedOffset(FixedOffset),
    Iana(Tz),
}

#[derive(Debug)]
pub enum ParsedDatetime {
    WithTimezone(DateTime<Utc>),
    Naive(NaiveDateTime),
}
