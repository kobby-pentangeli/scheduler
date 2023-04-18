use crate::Result;
use std::collections::HashMap;

pub struct ToDo {
    // Stores a key-value pair of a to-do item and its status
    // using the built-in `HashMap` of Rust.
    map: HashMap<String, bool>,
}

impl ToDo {
    pub fn create_new() -> Result<Self> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;

        match serde_json::from_reader(f) {
            Ok(map) => Ok(ToDo { map }),
            Err(e) if e.is_eof() => Ok(ToDo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("There was a problem: {}.", e),
        }
    }

    pub fn insert(&mut self, key: String) {
        // Inserts a new to-do activity in the map while
        // setting the status/state of the activity to `true` by default
        self.map.insert(key, true);
    }

    pub fn save(self) -> Result<()> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("db.json")?;
        serde_json::to_writer_pretty(f, &self.map)?;

        Ok(())
    }

    pub fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(val) => {
                *val = false;
                Some(())
            }
            None => None,
        }
    }
}
