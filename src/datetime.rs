use crate::error::TimeParseError;
use crate::formats::default_formats;
use crate::types::{EpochKind, ParsedDatetime, TimeZoneParsed};
use crate::tz::parse_timezone_str;
use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};

/// Attempts to detect the kind of epoch (timestamp) based on the length of the string.
///
/// This function assumes:
/// - 10 digits → seconds
/// - 13 digits → milliseconds
/// - 16 digits → microseconds
/// - 19 digits → nanoseconds
///
/// # Arguments
/// * `s` - A string containing a numeric epoch.
///
/// # Returns
/// * `Some(EpochKind)` if the string matches a known epoch length.
/// * `None` if it doesn't match any known format.
pub fn detect_epoch_kind(s: &str) -> Option<EpochKind> {
    match s.len() {
        10 => Some(EpochKind::Seconds),
        13 => Some(EpochKind::Milliseconds),
        16 => Some(EpochKind::Microseconds),
        19 => Some(EpochKind::Nanoseconds),
        _ => None,
    }
}

/// Parses a datetime string into a `DateTime<Utc>`, accepting a wide variety of formats.
///
/// This function supports:
/// - Unix epoch (seconds, milliseconds, microseconds, nanoseconds)
/// - ISO 8601, RFC 3339, RFC 2822
/// - Flexible date/time formats (with optional European preference)
/// - Fallback timezone if input has no timezone
///
/// # Arguments
/// * `s` - The input datetime string.
/// * `fallback_tz` - Timezone used if input is naive (e.g., `Asia/Jakarta`, `+07:00`, `UTC`).
/// * `prefer_eu` - If true, will try European formats first (e.g., DD-MM-YYYY).
/// * `usr_custom_formats` - Optional list of custom formats to try before defaults.
///
/// # Returns
/// * `Ok(DateTime<Utc>)` - Normalized UTC datetime.
/// * `Err(TimeParseError)` - If parsing fails or time is ambiguous.
pub fn utcize<S>(
    s: &str,
    fallback_tz: &str,
    prefer_eu: bool,
    usr_custom_formats: Option<&[S]>,
) -> Result<DateTime<Utc>, TimeParseError>
where
    S: AsRef<str>,
{
    let s = s.trim();

    // === Epoch numeric ===
    if s.chars().all(|c| c.is_numeric()) {
        if let Ok(num) = s.parse::<i64>() {
            if let Some(kind) = detect_epoch_kind(s) {
                let dt_opt = match kind {
                    EpochKind::Seconds => Utc.timestamp_opt(num, 0).single(),
                    EpochKind::Milliseconds => {
                        let secs = num / 1000;
                        let nsecs = ((num % 1000) * 1_000_000) as u32;
                        Utc.timestamp_opt(secs, nsecs).single()
                    }
                    EpochKind::Microseconds => {
                        let secs = num / 1_000_000;
                        let nsecs = ((num % 1_000_000) * 1_000) as u32;
                        Utc.timestamp_opt(secs, nsecs).single()
                    }
                    EpochKind::Nanoseconds => {
                        let secs = num / 1_000_000_000;
                        let nsecs = (num % 1_000_000_000) as u32;
                        Utc.timestamp_opt(secs, nsecs).single()
                    }
                };

                return dt_opt.ok_or_else(|| {
                    TimeParseError::InvalidInput("Epoch out of valid range".into())
                });
            }
        }
    }

    // === RFC 3339 / 2822 ===
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Ok(dt.with_timezone(&Utc));
    }
    if let Ok(dt) = DateTime::parse_from_rfc2822(s) {
        return Ok(dt.with_timezone(&Utc));
    }

    // === Custom / Flexible format ===
    match parse_datetime_flexible(s, prefer_eu, usr_custom_formats)? {
        ParsedDatetime::WithTimezone(dt) => Ok(dt),
        ParsedDatetime::Naive(naive) => {
            match parse_timezone_str(fallback_tz)? {
                TimeZoneParsed::FixedOffset(offset) => {
                    let dt = offset
                        .from_local_datetime(&naive)
                        .single()
                        .or_else(|| Some(offset.from_utc_datetime(&naive)))
                        .ok_or_else(|| {
                            TimeParseError::InvalidInput("Failed to resolve datetime".into())
                        })?;
                    Ok(dt.with_timezone(&Utc))
                }
                TimeZoneParsed::Iana(tz) => match tz.from_local_datetime(&naive) {
                    chrono::LocalResult::Single(dt) => Ok(dt.with_timezone(&Utc)),
                    chrono::LocalResult::Ambiguous(a, b) => Err(TimeParseError::AmbiguousTime {
                        datetime: naive,
                        options: vec![a.with_timezone(&Utc), b.with_timezone(&Utc)],
                    }),
                    chrono::LocalResult::None => Err(TimeParseError::InvalidInput(format!(
                        "Nonexistent local time due to DST: {} in {}",
                        naive, tz
                    ))),
                },
            }
        }
    }
}

/// Tries to parse a datetime string using custom and default formats.
///
/// If the format includes timezone offset (e.g., `%z` or `%:z`), it returns a fully qualified UTC datetime.
/// If the format is naive (no timezone info), it returns a `NaiveDateTime` which requires a fallback.
///
/// # Arguments
/// * `s` - Input datetime string.
/// * `prefer_eu` - Use European-style formats first (DD-MM-YYYY).
/// * `custom_formats` - Optional list of custom formats.
///
/// # Returns
/// * `Ok(ParsedDatetime::WithTimezone)` if the string includes timezone information.
/// * `Ok(ParsedDatetime::Naive)` if timezone is missing and fallback is needed.
/// * `Err(TimeParseError)` if no format matched.
pub fn parse_datetime_flexible<S>(
    s: &str,
    prefer_eu: bool,
    custom_formats: Option<&[S]>,
) -> Result<ParsedDatetime, TimeParseError>
where
    S: AsRef<str>,
{
    let mut formats: Vec<String> = vec![];

    if let Some(customs) = custom_formats {
        formats.extend(customs.iter().map(|s| s.as_ref().to_string()));
    }

    formats.extend(default_formats(prefer_eu).into_iter().map(|s| s.to_string()));

    for fmt in formats {
        let fmt_str = fmt.as_str();

        if let Ok(dt) = DateTime::parse_from_str(s, fmt_str) {
            return Ok(ParsedDatetime::WithTimezone(dt.with_timezone(&Utc)));
        }

        if !fmt_str.contains("%z") && !fmt_str.contains("%:z") {
            if let Ok(ndt) = NaiveDateTime::parse_from_str(s, fmt_str) {
                return Ok(ParsedDatetime::Naive(ndt));
            }

            if let Ok(date) = NaiveDate::parse_from_str(s, fmt_str) {
                if let Some(ndt) = date.and_hms_opt(0, 0, 0) {
                    return Ok(ParsedDatetime::Naive(ndt));
                }
            }
        }
    }

    Err(TimeParseError::InvalidInput(format!("No matching format found for: '{}'", s)))
}
