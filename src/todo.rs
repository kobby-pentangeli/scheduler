use crate::{error::Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;

/// Represents a `todo` list
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    // A map of `todo` items, where the keys represent the items
    // and the values are booleans indicating whether the items are completed.
    entries: HashMap<String, bool>,
}

impl Todo {
    /// Creates a new `Todo`.
    ///
    /// If a file named "db.json" exists, it attempts to read from it and deserialize
    /// the data into a `Todo` struct. Otherwise, an empty `Todo` struct is created.
    pub fn new() -> Result<Self> {
        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;

        match serde_json::from_reader(f) {
            Ok(entries) => Ok(Self { entries }),
            Err(e) if e.is_eof() => Ok(Self {
                entries: HashMap::new(),
            }),
            Err(e) => Err(Error::SerdeJsonSerializeError(e)),
        }
    }

    /// Inserts a key-value pair into the map of `todo` entries.
    ///
    /// If the map did not have this key present, `None` is returned.
    /// If the map did have this key present, the value is updated, and
    /// the old value is returned. The key is not updated, though.
    pub fn insert(&mut self, key: &str) -> Option<bool> {
        self.entries.insert(key.to_string(), true)
    }

    /// Saves the current state of the `Todo` struct to a file named "db.json".
    pub fn save(self) -> Result<()> {
        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("db.json")?;
        Ok(serde_json::to_writer_pretty(f, &self.entries)?)
    }

    /// Marks a `todo` item as completed.
    pub fn complete(&mut self, key: &str) -> Option<()> {
        match self.entries.get_mut(key) {
            Some(val) => {
                *val = false;
                Some(())
            }
            None => None,
        }
    }

    /// Iterates over the `todo` entries, returning each key along with its status.
    /// If the value is true, it's considered incomplete, and if it's false, it's considered complete.
    pub fn read(&self) -> HashMap<&String, String> {
        let mut entries = HashMap::new();
        for (key, value) in &self.entries {
            let status = if *value {
                "Incomplete".to_string()
            } else {
                "Complete".to_string()
            };
            entries.insert(key, status);
        }
        entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_insert() {
        let mut todo = Todo::new().unwrap();
        assert_eq!(todo.insert("Task 1"), None);
        assert_eq!(todo.entries.len(), 1);
        assert_eq!(todo.insert("Task 1"), Some(true));
        assert_eq!(todo.entries.len(), 1);
    }

    #[test]
    fn test_todo_complete() {
        let mut todo = Todo::new().unwrap();
        todo.insert("Task 1");
        assert_eq!(todo.complete("Task 2"), None);
        assert_eq!(todo.complete("Task 1"), Some(()));
        assert_eq!(todo.entries.get("Task 1"), Some(&false));
    }

    #[test]
    fn test_todo_save() {
        let mut todo = Todo::new().unwrap();
        todo.insert("Task 1");
        todo.save().unwrap();
        let new_todo = Todo::new().unwrap();
        assert_eq!(new_todo.entries.get("Task 1"), Some(&true));
    }
}
