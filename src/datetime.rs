use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Tz;
use crate::error::TimeParseError;
use crate::types::{EpochKind, ParsedDatetime, TimeInput, TimeZoneParsed};

pub fn detect_epoch_kind(s: &str) -> Option<EpochKind> {
    match s.len() {
        10 => Some(EpochKind::Seconds),
        13 => Some(EpochKind::Milliseconds),
        16 => Some(EpochKind::Microseconds),
        19 => Some(EpochKind::Nanoseconds),
        _ => None,
    }
}

/// Main function
pub fn normalize_datetime<S>(
    s: &str,
    fallback_tz: &str,
    prefer_eu: bool,
    usr_custom_formats: Option<&[S]>,
) -> Result<TimeInput, TimeParseError>
where
    S: AsRef<str>,
{
    let s = s.trim();

    // === Epoch timestamp ===
    if s.chars().all(|c| c.is_numeric()) {
        if let Ok(num) = s.parse::<i64>() {
            if let Some(kind) = detect_epoch_kind(s) {
                let dt_opt = match kind {
                    EpochKind::Seconds => {
                        if (1_000_000_000..=3_250_000_000).contains(&num) {
                            DateTime::<Utc>::from_timestamp(num, 0)
                        } else {
                            None
                        }
                    }
                    EpochKind::Milliseconds => {
                        if (1_000_000_000_000..=3_250_000_000_000).contains(&num) {
                            let secs = num / 1000;
                            let nsecs = ((num % 1000) * 1_000_000) as u32;
                            DateTime::<Utc>::from_timestamp(secs, nsecs)
                        } else {
                            None
                        }
                    }
                    EpochKind::Microseconds => {
                        if (1_000_000_000_000_000..=3_250_000_000_000_000).contains(&num) {
                            let secs = num / 1_000_000;
                            let nsecs = ((num % 1_000_000) * 1_000) as u32;
                            DateTime::<Utc>::from_timestamp(secs, nsecs)
                        } else {
                            None
                        }
                    }
                    EpochKind::Nanoseconds => {
                        if (1_000_000_000_000_000_000..=3_250_000_000_000_000_000).contains(&num) {
                            let secs = num / 1_000_000_000;
                            let nsecs = (num % 1_000_000_000) as u32;
                            DateTime::<Utc>::from_timestamp(secs, nsecs)
                        } else {
                            None
                        }
                    }
                };

                if let Some(dt) = dt_opt {
                    println!("\n{:?} -> Epoch {:?} ::--> DateTime::<Utc>::from_timestamp", dt, Some(kind));
                    return Ok(TimeInput::Iso(dt));
                } else {
                    return Err(TimeParseError::InvalidInput("Epoch out of valid range".into()));
                }
            }
        }
    }

    // === RFC 3339 ===
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        println!("\n{:?} -> DateTime::parse_from_rfc3339 ", dt);
        return Ok(TimeInput::Iso(dt.with_timezone(&Utc)));
    }

    // === RFC 2822 ===
    if let Ok(dt) = DateTime::parse_from_rfc2822(s) {
        println!("\n{:?} -> DateTime::parse_from_rfc2822 ", dt);
        return Ok(TimeInput::Iso(dt.with_timezone(&Utc)));
    }

    match parse_datetime_flexible(s, prefer_eu, usr_custom_formats)? {
        ParsedDatetime::WithTimezone(dt) => {
            Ok(TimeInput::Iso(dt))
        }

        ParsedDatetime::Naive(naive) => {
            match parse_timezone_str(fallback_tz)? {
                TimeZoneParsed::FixedOffset(offset) => {
                    let dt = offset
                        .from_local_datetime(&naive)
                        .single()
                        .unwrap_or_else(|| offset.from_utc_datetime(&naive));
                    Ok(TimeInput::Iso(dt.with_timezone(&Utc)))
                }

                TimeZoneParsed::Iana(tz) => match tz.from_local_datetime(&naive) {
                    chrono::LocalResult::Single(dt) => {
                        println!("{:?} parse_with_tz :: LocalResult::Single ", tz);
                        Ok(TimeInput::Iso(dt.with_timezone(&Utc)))
                    }
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

pub fn parse_datetime_flexible<S>(
    s: &str,
    prefer_eu: bool,
    custom_formats: Option<&[S]>,
) -> Result<ParsedDatetime, TimeParseError>
where
    S: AsRef<str>,
{
    let mut formats = vec![];

    // Add custom formats (if any)
    if let Some(customs) = custom_formats {
        formats.extend(customs.iter().map(|s| s.as_ref().to_string()));
    }

    // Add default formats depending on preference
    if prefer_eu {
        formats.extend(vec![
            "%d-%m-%Y %H:%M:%S%z".into(),   // 01-06-2045 10:00:00+0700
            "%d-%m-%Y %H:%M:%S".into(),     // 01-06-2045 10:00:00
            "%d-%m-%Y".into(),              // 01-06-2045
        ]);
    } else {
        formats.extend(vec![
            "%m-%d-%Y %H:%M:%S%z".into(),   // 06-01-2045 10:00:00+0700
            "%m-%d-%Y %H:%M:%S".into(),     // 06-01-2045 10:00:00
            "%m-%d-%Y".into(),              // 06-01-2045
        ]);
    }

    formats.extend(vec![
        "%Y-%m-%dT%H:%M:%S%z".into(),      // 2045-06-01T10:00:00+0700
        "%Y-%m-%dT%H:%M:%S%:z".into(),     // 2045-06-01T10:00:00+07:00
        "%Y-%m-%d %H:%M:%S%z".into(),
        "%Y-%m-%d %H:%M:%S".into(),
        "%Y-%m-%d".into(),
    ]);

    for fmt in formats {
        let fmt_str = fmt.as_str();
        // Try parsing with timezone first
        if let Ok(dt) = DateTime::parse_from_str(s, fmt_str) {
            return Ok(ParsedDatetime::WithTimezone(dt.with_timezone(&Utc)));
        }

        // If no timezone format, try as Naive
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

    Err(TimeParseError::InvalidInput(format!(
        "No matching format found for: '{}'",
        s
    )))
}

pub fn parse_timezone_str(tz_str: &str) -> Result<TimeZoneParsed, TimeParseError> {
    if tz_str.starts_with('+') || tz_str.starts_with('-') {
        if let Ok(offset) = tz_str.parse::<FixedOffset>() {
            return Ok(TimeZoneParsed::FixedOffset(offset).into());
        }
    }
    match tz_str.parse::<Tz>() {
        Ok(tz) => Ok(TimeZoneParsed::Iana(tz).into()),
        Err(e) => Err(TimeParseError::InvalidInput(format!("Timezone '{}' is invalid fixed offset or IANA timezone:::{}", tz_str, e)))
    }
}
