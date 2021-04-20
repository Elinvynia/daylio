//! [![ci-badge][]][ci] [![docs-badge][]][docs] [![crate-version]][crate-link]
//!
//! # daylio
//!
//! A simple Daylio diary parsing and utility library.
//!
//! ## Sample Usage
//!
//! Parse a default configuration .csv file into entries.
//!
//! ```rust
//!let entries = daylio::parse("data.csv");
//! ```
//!
//! [ci]: https://github.com/Elinvynia/daylio/actions?query=workflow%3ARust
//! [ci-badge]: https://img.shields.io/github/workflow/status/Elinvynia/daylio/Rust/master?style=flat-square
//! [docs]: https://docs.rs/daylio
//! [docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square
//! [crate-link]: https://crates.io/crates/daylio
//! [crate-version]: https://img.shields.io/crates/v/daylio.svg?style=flat-square

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]

mod error;
pub use error::DaylioError;
mod mood;
pub use mood::{Mood, MoodConfig};
mod parse;
pub use parse::Entry;
use parse::Row;

use chrono::prelude::*;
use csv::ReaderBuilder;
use std::path::Path;

/// Parse a `.csv` file into Daylio diary entries using the default [`MoodConfig`].
pub fn parse<P: AsRef<Path>>(path: P) -> Result<Vec<Entry>, DaylioError> {
    let config = MoodConfig::default();
    parse_with_config(path, config)
}

/// Parse a `.csv` file into Daylio diary entries using your custom [`MoodConfig`].
pub fn parse_with_config<P: AsRef<Path>>(path: P, config: MoodConfig) -> Result<Vec<Entry>, DaylioError> {
    let reader = ReaderBuilder::new().delimiter(b',').quote(b'"').from_path(path)?;
    let mut entries = vec![];

    for row in reader.into_deserialize() {
        let row: Row = row?;

        let datetime;
        let test = NaiveDateTime::parse_from_str(&format!("{} {:0>8}", row.full_date, row.time), "%F %I:%M %P");
        if test.is_err() {
            datetime = NaiveDateTime::parse_from_str(&format!("{} {}", row.full_date, row.time), "%F %H:%M")?;
        } else {
            datetime = test?;
        }
        let mood = config.get(&row.mood).ok_or(DaylioError::MoodNotFound)?.clone();
        let activities = row.activities.split('|').map(|x| x.to_string()).collect();
        let notes = row.note;

        let entry = Entry {
            datetime,
            mood,
            activities,
            notes,
        };
        entries.push(entry)
    }

    Ok(entries)
}
