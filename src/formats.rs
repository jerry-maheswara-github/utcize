/// Returns a list of default datetime format strings for parsing.
///
/// The formats returned depend on the `prefer_eu` flag:
/// - If `true`, European-style date formats (day-month-year) are preferred.
/// - If `false`, US-style date formats (month-day-year) are preferred.
///
/// This list also includes common ISO 8601 variants, compact ISO formats,
/// ISO week dates, and RFC date formats for robust datetime parsing.
///
/// # Arguments
///
/// * `prefer_eu` - A boolean indicating whether to prefer European date formats.
///
/// # Returns
///
/// A `Vec<&'static str>` containing format specifiers compatible with `chrono`'s parsing functions.
///
/// # Examples
///
/// ```
/// use utcize::formats::default_formats;
/// let formats = default_formats(true);  // European date formats preferred
/// let formats_us = default_formats(false); // US date formats preferred
/// ```
pub fn default_formats(prefer_eu: bool) -> Vec<&'static str> {
    let mut formats = vec![];

    if prefer_eu {
        formats.extend(vec![
            "%d-%m-%Y %H:%M:%S%z",      // 01-06-2045 10:00:00+0700
            "%d-%m-%Y %H:%M:%S",        // 01-06-2045 10:00:00
            "%d-%m-%Y %H:%M",           // 01-06-2045 10:00
            "%d-%m-%Y",                 // 01-06-2045
            "%d/%m/%Y %H:%M:%S",        // 01/06/2045 10:00:00
            "%d/%m/%Y %H:%M",           // 01/06/2045 10:00
            "%d/%m/%Y",                 // 01/06/2045
            "%d.%m.%Y %H:%M:%S",        // 01.06.2045 10:00:00
            "%d.%m.%Y",                 // 01.06.2045
            "%d %b %Y",                 // 01 Jun 2045
            "%d %B %Y",                 // 01 June 2045
        ]);
    } else {
        formats.extend(vec![
            "%m-%d-%Y %H:%M:%S%z",      // 06-01-2045 10:00:00+0700
            "%m-%d-%Y %H:%M:%S",        // 06-01-2045 10:00:00
            "%m-%d-%Y %H:%M",           // 06-01-2045 10:00
            "%m-%d-%Y",                 // 06-01-2045
            "%m/%d/%Y %H:%M:%S",        // 06/01/2045 10:00:00
            "%m/%d/%Y %H:%M",           // 06/01/2045 10:00
            "%m/%d/%Y",                 // 06/01/2045
            "%B %d, %Y",                // June 1, 2045
            "%b %d, %Y",                // Jun 1, 2045
        ]);
    }

    formats.extend(vec![
        // ISO 8601 variants
        "%Y-%m-%dT%H:%M:%S%z",           // 2045-06-01T10:00:00+0700
        "%Y-%m-%dT%H:%M:%S%:z",          // 2045-06-01T10:00:00+07:00
        "%Y-%m-%d %H:%M:%S%z",           // 2023-06-01 10:00:00+0000
        "%Y-%m-%d %H:%M:%S",             // 2023-06-01 10:00:00
        "%Y-%m-%d",                      // 2023-06-01
        // ISO 8601 with fractional seconds
        "%Y-%m-%dT%H:%M:%S%.f%z",        // 2045-06-01T10:00:00.123456+0700
        "%Y-%m-%dT%H:%M:%S%.f%:z",       // 2045-06-01T10:00:00.123456+07:00
        "%Y-%m-%d %H:%M:%S%.f%z",        // 2045-06-01 10:00:00.123456+0700
        "%Y-%m-%d %H:%M:%S%.f",          // 2045-06-01 10:00:00.123456
        // ISO 8601 compact (no separators)
        "%Y%m%dT%H%M%S%z",               // 20230601T100000+0000
        "%Y%m%dT%H%M%S",                 // 20230601T100000
        "%Y%m%d%H%M%S%Z",                // 20230601100000UTC
        "%Y%m%d%H%M%S",                  // 20230601100000
        // ISO week date
        "%G-W%V-%u",                     // 2023-W22-4 (ISO week date)
        "%G-W%V",                        // 2023-W22
        // RFC 822 / 1123 / 2822 variants
        "%a, %d %b %Y %H:%M:%S %z",      // Thu, 01 Jun 2023 10:00:00 +0700
        "%d %b %Y %H:%M:%S %z",          // 01 Jun 2023 10:00:00 +0700
    ]);

    formats
}
