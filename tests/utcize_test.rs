#[cfg(test)]
mod tests {
    use utcize::types::TimeZoneParsed;
    use utcize::tz::parse_timezone_str;

    #[test]
    fn test_valid_utc() {
        let tz = parse_timezone_str("UTC").unwrap();
        match tz {
            TimeZoneParsed::FixedOffset(offset) => assert_eq!(offset.local_minus_utc(), 0),
            _ => panic!("Expected FixedOffset"),
        }

        let tz_lower = parse_timezone_str("utc").unwrap();
        match tz_lower {
            TimeZoneParsed::FixedOffset(offset) => assert_eq!(offset.local_minus_utc(), 0),
            _ => panic!("Expected FixedOffset"),
        }
    }

    #[test]
    fn test_valid_fixed_offset() {
        let tz = parse_timezone_str("+07:00").unwrap();
        match tz {
            TimeZoneParsed::FixedOffset(offset) => assert_eq!(offset.local_minus_utc(), 7 * 3600),
            _ => panic!("Expected FixedOffset"),
        }

        let tz2 = parse_timezone_str("-0830").unwrap();
        match tz2 {
            TimeZoneParsed::FixedOffset(offset) => assert_eq!(offset.local_minus_utc(), -8 * 3600 - 30 * 60),
            _ => panic!("Expected FixedOffset"),
        }
    }

    #[test]
    fn test_valid_iana() {
        let tz = parse_timezone_str("Asia/Jakarta").unwrap();
        match tz {
            TimeZoneParsed::Iana(tz) => assert_eq!(tz.name(), "Asia/Jakarta"),
            _ => panic!("Expected IANA timezone"),
        }
    }

    #[test]
    fn test_invalid_fixed_offset_format() {
        let err = parse_timezone_str("0800").unwrap_err();
        assert!(
            format!("{}", err).contains("Invalid IANA timezone format") ||
                format!("{}", err).contains("Invalid fixed offset format")
        );
    }

    #[test]
    fn test_invalid_iana_format() {
        let err = parse_timezone_str("/Asia/Jakarta").unwrap_err();
        assert!(format!("{}", err).contains("Invalid IANA timezone format"));

        let err2 = parse_timezone_str("Asia/Jakarta/").unwrap_err();
        assert!(format!("{}", err2).contains("Invalid IANA timezone format"));

        let err3 = parse_timezone_str("Asia").unwrap_err();
        assert!(format!("{}", err3).contains("Invalid IANA timezone format"));
    }

    #[test]
    fn test_unknown_iana_timezone() {
        let err = parse_timezone_str("Invalid/Zone").unwrap_err();
        assert!(format!("{}", err).contains("Unknown IANA timezone"));
    }
}
