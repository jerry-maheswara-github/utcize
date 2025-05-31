use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use chrono_tz::Tz;
/// Represents the unit precision of a Unix timestamp.
///
/// Used to detect the scale of numeric epoch values when parsing.
#[derive(Debug, PartialEq, PartialOrd)]
pub enum EpochKind {
    /// Timestamp in seconds since the Unix epoch.
    Seconds,
    /// Timestamp in milliseconds since the Unix epoch.
    Milliseconds,
    /// Timestamp in microseconds since the Unix epoch.
    Microseconds,
    /// Timestamp in nanoseconds since the Unix epoch.
    Nanoseconds,
}

/// Represents a parsed timezone, either as a fixed offset or an IANA timezone.
///
/// `FixedOffset` is for numeric offsets like `+07:00`.
/// `Iana` is for named timezones like `"Europe/Berlin"`.
#[derive(Debug)]
pub enum TimeZoneParsed {
    /// Fixed UTC offset timezone.
    FixedOffset(FixedOffset),
    /// IANA timezone identifier.
    Iana(Tz),
}

/// Represents a parsed datetime, either with a timezone (converted to UTC)
/// or a naive datetime without timezone information.
#[derive(Debug)]
pub enum ParsedDatetime {
    /// Datetime with timezone information, normalized to UTC.
    WithTimezone(DateTime<Utc>),
    /// Naive datetime without timezone.
    Naive(NaiveDateTime),
}
