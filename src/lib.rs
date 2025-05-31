//! # utcize
//! **Normalize any datetime input to UTC. Automatically.**
//! 
//! `utcize` is a lightweight Rust library for normalizing various datetime inputs into
//! a `DateTime<Utc>` using the [`chrono`](https://crates.io/crates/chrono) crate.
//!
//! ## Features
//!
//! - Automatic format detection (RFC 3339, RFC 2822, ISO 8601, Unix timestamps: seconds, milliseconds, microseconds, nanoseconds).
//! - Supports both European-style (`dd-mm-yyyy`) and US-style (`mm-dd-yyyy`) formats.
//! - Handles fixed timezone offsets (e.g., `+07:00`, `-0800`) and IANA timezones (e.g., `Asia/Jakarta`, `Europe/Berlin`).
//! - Fallback timezone support for naive datetime strings (without timezone).
//! - Ambiguity handling for local times during daylight saving transitions.
//! - Easy to extend with custom formats.
//!
//! ## Example
//!
//! ```rust
//!  use utcize::datetime::utcize;
//!
//!  let dt = utcize::<&str>("2023-06-01 10:00:00", "Asia/Jakarta", false, None).unwrap();
//!  assert_eq!(dt.to_rfc3339(), "2023-06-01T03:00:00+00:00");
//! 
//!  let res = utcize::<&str>("2023-06-01 10:00:00", "+07:00", false, None).unwrap();
//!  assert_eq!(res.to_rfc3339(), "2023-06-01T03:00:00+00:00");
//! 
//!  let dt = utcize::<&str>("01-06-2023 10:00:00", "Europe/Paris", true, None).unwrap();
//!  assert_eq!(dt.to_rfc3339(), "2023-06-01T08:00:00+00:00");
//! 
//!  let custom_format = ["%Y|%m|%d %H:%M", "%d.%B.%Y %H:%M"];
//!
//!  let dt = utcize("2023|06|01 10:00", "Asia/Jakarta", false, Some(&custom_format)).unwrap();
//!  assert_eq!(dt.to_rfc3339(), "2023-06-01T03:00:00+00:00");
//! 
//!  let dt = utcize("2023|06|01 10:00", "+07:00", false, Some(&custom_format)).unwrap();
//!  assert_eq!(dt.to_rfc3339(), "2023-06-01T03:00:00+00:00");
//!     
//!  let dt = utcize("01.June.2023 10:00", "Europe/Berlin", true, Some(&custom_format)).unwrap();
//!  assert_eq!(dt.to_rfc3339(), "2023-06-01T08:00:00+00:00")
//! ```
//! 
//! ---
//! 
//! ## License
//! 
//!  -  Licensed under Apache License, Version 2.0 [LICENSE](http://www.apache.org/licenses/LICENSE-2.0.txt)
//! 
//!  ---
//! 
//!  ## Author
//! 
//!  - Created and maintained by [Jerry Maheswara](https://github.com/jerry-maheswara-github)
//! 
//!  ---
//! 
//!  ## Built with Love in Rust
//! 
//!  - This project is built with ❤️ using **Rust** — a systems programming language that is safe, fast, and concurrent. Rust is the perfect choice for building reliable and efficient applications.
//! 
//!  ---
 

/// Main module for parsing and normalizing datetime strings to UTC.
pub mod datetime;

/// Error definitions for datetime parsing and conversion.
pub mod error;

/// Internal data types representing parsed time, timezone kinds, and epoch types.
pub mod types;

/// Collection of datetime format strings used for flexible parsing.
pub mod formats;

/// Functions for parsing and validating fixed and IANA timezones.
pub mod tz;
