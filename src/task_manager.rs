use crate::{error::Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;

/// Represents a container for scheduled tasks
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TaskManager {
    // A map of scheduled tasks, where the keys represent the items
    // and the values are booleans indicating whether the tasks are completed.
    tasks: HashMap<String, bool>,
}

impl TaskManager {
    /// Creates a new `TaskManager`.
    ///
    /// If a file named "db.json" exists, it attempts to read from it and deserialize
    /// the data into a `TaskManager`. Otherwise, an empty `TaskManager` is created.
    pub fn new() -> Result<Self> {
        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;

        match serde_json::from_reader(f) {
            Ok(tasks) => Ok(Self { tasks }),
            Err(e) if e.is_eof() => Ok(Self {
                tasks: HashMap::new(),
            }),
            Err(e) => Err(Error::SerdeJsonSerializeError(e)),
        }
    }

    /// Inserts a key-value pair into the map of tasks.
    ///
    /// If the map did not have this key present, `None` is returned.
    /// If the map did have this key present, the value is updated, and
    /// the old value is returned. The key is not updated, though.
    pub fn insert(&mut self, key: &str) -> Option<bool> {
        self.tasks.insert(key.to_string(), true)
    }

    /// Saves the current state of the `TaskManager` to a file named "db.json".
    pub fn save(&self) -> Result<()> {
        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("db.json")?;
        Ok(serde_json::to_writer_pretty(f, &self.tasks)?)
    }

    /// Marks a task as completed.
    pub fn complete(&mut self, key: &str) -> Option<()> {
        match self.tasks.get_mut(key) {
            Some(val) => {
                *val = false;
                Some(())
            }
            None => None,
        }
    }

    /// Iterates over the tasks, returning each key along with its status.
    /// If the value is true, it's considered incomplete, and if it's false, it's considered complete.
    pub fn read(&self) -> HashMap<&String, String> {
        let mut tasks = HashMap::new();
        for (key, value) in &self.tasks {
            let status = if *value {
                "Incomplete".to_string()
            } else {
                "Complete".to_string()
            };
            tasks.insert(key, status);
        }
        tasks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_manager_insert() {
        let mut task_manager = TaskManager::new().unwrap();
        assert_eq!(task_manager.insert("Task 1"), None);
        assert_eq!(task_manager.tasks.len(), 1);
        assert_eq!(task_manager.insert("Task 1"), Some(true));
        assert_eq!(task_manager.tasks.len(), 1);
    }

    #[test]
    fn test_task_manager_complete() {
        let mut task_manager = TaskManager::new().unwrap();
        task_manager.insert("Task 1");
        assert_eq!(task_manager.complete("Task 2"), None);
        assert_eq!(task_manager.complete("Task 1"), Some(()));
        assert_eq!(task_manager.tasks.get("Task 1"), Some(&false));
    }

    #[test]
    fn test_task_manager_save() {
        let mut task_manager = TaskManager::new().unwrap();
        task_manager.insert("Task 1");
        task_manager.save().unwrap();
        let new_task_manager = TaskManager::new().unwrap();
        assert_eq!(new_task_manager.tasks.get("Task 1"), Some(&true));
    }
}
