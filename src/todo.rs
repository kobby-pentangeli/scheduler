use crate::{error::Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    entries: HashMap<String, bool>,
}

impl Todo {
    pub fn new() -> Result<Self> {
        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;

        match serde_json::from_reader(f) {
            Ok(entries) => Ok(Self { entries }),
            Err(e) if e.is_eof() => Ok(Self::default()),
            Err(e) => Err(Error::SerdeJsonSerializeError(e)),
        }
    }

    /// Inserts a key-value pair into the map of journal entries.
    ///
    /// If the map did not have this key present, `None` is returned.
    /// If the map did have this key present, the value is updated, and
    /// the old value is returned. The key is not updated, though.
    pub fn insert(&mut self, key: &str) -> Option<bool> {
        self.entries.insert(key.to_string(), true)
    }

    pub fn save(self) -> Result<()> {
        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("db.json")?;
        Ok(serde_json::to_writer_pretty(f, &self.entries)?)
    }

    pub fn complete(&mut self, key: &str) -> Option<()> {
        match self.entries.get_mut(key) {
            Some(val) => {
                *val = false;
                Some(())
            }
            None => None,
        }
    }

    /// Iterates over the entries map, returning each key along with its status.
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
