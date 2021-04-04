// Necessary for pretty doc links.
#[allow(unused_imports)]
use crate::{Mood, MoodConfig};
use chrono::ParseError as ChronoError;
use csv::Error as CsvError;
use std::fmt;

/// Errors that can occur while working with this library.
#[derive(Debug)]
pub enum DaylioError {
    /// Failed to parse the `.csv` file.
    CsvError(CsvError),
    /// Failed to parse the time in a row.
    ChronoError(ChronoError),
    /// A [`Mood`] was not found in the provided [`MoodConfig`].
    MoodNotFound,
}

impl std::error::Error for DaylioError {}

impl fmt::Display for DaylioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DaylioError::*;
        let message = match self {
            CsvError(e) => format!("CSV parsing error: {}", e),
            ChronoError(e) => format!("Chrono parsing error: {}", e),
            MoodNotFound => "Provided mood not found in config.".into(),
        };

        write!(f, "{}", message)
    }
}

impl From<CsvError> for DaylioError {
    fn from(e: CsvError) -> Self {
        DaylioError::CsvError(e)
    }
}

impl From<ChronoError> for DaylioError {
    fn from(e: ChronoError) -> Self {
        DaylioError::ChronoError(e)
    }
}
