use chrono::{FixedOffset};
use chrono_tz::Tz;
use crate::error::TimeParseError;
use crate::types::TimeZoneParsed;

/// Parses a timezone string and returns a [`TimeZoneParsed`] enum indicating either a fixed offset
/// or an IANA timezone (e.g., `Asia/Jakarta`, `Europe/Berlin`).
///
/// # Supported formats
///
/// - `"UTC"` (case-insensitive): returns a fixed offset of `+00:00`.
/// - Fixed offset: `+07:00`, `-0800`, etc. (parsed using [`FixedOffset`]).
/// - IANA timezone: must be in the format `"Region/City"`.
///
/// # Errors
///
/// Returns [`TimeParseError::InvalidInput`] if:
/// - The string is not a valid timezone format,
/// - The fixed offset format is invalid,
/// - The IANA string is malformed or unrecognized.
///
/// # Examples
///
/// ```
/// use utcize::tz::parse_timezone_str;
///
/// let tz = parse_timezone_str("Asia/Jakarta").unwrap();
/// let utc = parse_timezone_str("UTC").unwrap();
/// let offset = parse_timezone_str("+07:00").unwrap();
/// ```
///
/// [`FixedOffset`]: FixedOffset
/// [`TimeZoneParsed`]: TimeZoneParsed
/// [`TimeParseError::InvalidInput`]: TimeParseError
pub fn parse_timezone_str(tz_str: &str) -> Result<TimeZoneParsed, TimeParseError> {
    let tz_str = tz_str.trim();

    // UTC as a special case (fallback, common usage)
    if tz_str.eq_ignore_ascii_case("UTC") {
        return FixedOffset::east_opt(0)
            .map(TimeZoneParsed::FixedOffset)
            .ok_or_else(|| TimeParseError::InvalidInput("Invalid UTC offset".into()));
    }

    // Handle fixed offset: +07:00, -0800, etc.
    if tz_str.starts_with('+') || tz_str.starts_with('-') {
        if let Ok(offset) = tz_str.parse::<FixedOffset>() {
            return Ok(TimeZoneParsed::FixedOffset(offset));
        } else {
            return Err(TimeParseError::InvalidInput(format!(
                "Invalid fixed offset format: '{}'", tz_str
            )));
        }
    }

    // Must follow IANA format: "Region/Location"
    if !tz_str.contains('/') || tz_str.starts_with('/') || tz_str.ends_with('/') {
        return Err(TimeParseError::InvalidInput(format!(
            "Invalid IANA timezone format: '{}'", tz_str
        )));
    }

    // Try parse as IANA timezone
    match tz_str.parse::<Tz>() {
        Ok(tz) => Ok(TimeZoneParsed::Iana(tz)),
        Err(e) => Err(TimeParseError::InvalidInput(format!(
            "Unknown IANA timezone '{}': {}", tz_str, e
        ))),
    }
}
