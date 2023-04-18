use crate::{error::Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Journal {
    entries: HashMap<String, bool>,
}

impl Journal {
    pub fn new() -> Result<Self> {
        let f = std::fs::OpenOptions::new()
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

    pub fn insert(&mut self, key: &str) {
        // Inserts a new to-do activity in the map while
        // setting the status/state of the activity to `true` by default
        self.entries.insert(key.to_string(), true);
    }

    pub fn save(self) -> Result<()> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("db.json")?;
        serde_json::to_writer_pretty(f, &self.entries)?;

        Ok(())
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
}
