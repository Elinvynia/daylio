use crate::mood::Mood;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Row {
    pub(crate) full_date: String,
    pub(crate) date: String,
    pub(crate) weekday: String,
    pub(crate) time: String,
    pub(crate) mood: String,
    pub(crate) activities: String,
    pub(crate) note_title: String,
    pub(crate) note: String,
}

/// Represents an entry in the Daylio diary.
#[derive(Debug, Clone)]
pub struct Entry {
    /// The time this entry was created.
    pub datetime: NaiveDateTime,
    /// [`Mood`] connected to this entry.
    pub mood: Mood,
    /// Activities associated with this entry.
    pub activities: Vec<String>,
    /// Custom note associated with this entry.
    pub notes: String,
}
