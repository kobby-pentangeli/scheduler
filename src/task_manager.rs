use crate::{error::Error, Result};
use serde::{Deserialize, Serialize};
use std::str::FromStr
use std::{collections::HashMap, fs::File};
use std::fs::OpenOptions;

/// Represents a container for scheduled tasks
#[derive(Debug, Deserialize, Serialize)]
pub struct TaskManager {
    // A map of scheduled tasks, where the keys represent the tasks,
    // and the values are booleans indicating whether the tasks are completed.
    // A value `true` means the task is "completed".
    tasks: HashMap<Task, bool>,
}

/// Represents a single task
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize, Clone)]
pub struct Task {
    id: u32,
    description: String,
}

impl Task {
    pub fn new(task_descr: &str) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let id: u32 = rng.gen();

        Self {
            id,
            description: task_descr.to_string(),
        }
    }

    pub fn build_from_str(s: &str) -> Self {
        let task = match serde_json::from_str::<Self>(s) {
            Ok(val) => val,
            Err(e) => panic!("{}", e)
        };
        task
    }
}

impl ToString for Task {
    fn to_string(&self) -> String {
        format!(
            "Task {{ id: {}, description: {} }}",
            self.id, self.description
        )
    }
}

// impl FromStr for Task {
//     type Err = std::io::Error;

//     fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
//         match s.is_empty() {
//             false => {
//                 let task: Task = serde_json::from_str(s)?;
//                 Ok(task)
//             },
//             true => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "No task description provided")),
//         }
//     }
// }

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
            .open("db.json");

        match f {
            Ok(file) => {
                match serde_json::from_reader::<File, HashMap<String, bool>>(file) {
                    Ok(t) => {
                        let tasks = t
                        .iter()
                        .map(|(task, status)| (Task::build_from_str(task), *status))
                        .collect::<HashMap<String, bool>>();
                        Ok(Self { tasks })
                    },
                    Err(e) if e.is_eof() => Ok(Self {
                        tasks: HashMap::new(),
                    }),
                    Err(e) => Err(Error::SerdeJsonSerializeError(e)),
                }
            },
            Err(e) => Err(Error::FileCreateAndReadError(e))
        }
    }

    /// Inserts a key-value pair into the map of tasks.
    ///
    /// If the map did not have this key present, `None` is returned.
    /// If the map did have this key present, the value is updated, and
    /// the old value is returned. The key is not updated, though.
    pub fn insert(&mut self, task: &Task) -> Option<bool> {
        self.tasks.insert(task.clone(), false)
    }

    /// Saves the current state of the `TaskManager` to a file named "db.json".
    pub fn save(&self) -> Result<()> {
        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json");

        match f {
            Err(e) => Err(Error::FileCreateAndReadError(e)),
            Ok(file) => {
                let tasks = self
                    .tasks
                    .iter()
                    .map(|(task, status)| (task.to_string(), *status))
                    .collect::<HashMap<String, bool>>();

                match serde_json::to_writer_pretty(file, &tasks) {
                    Ok(_res) => Ok(()),
                    Err(e) => Err(Error::SerdeJsonSerializeError(e)),
                }
            }
        }
    }

    /// Marks a task as completed.
    pub fn complete(&mut self, task: &Task) -> Option<String> {
        match self.tasks.get_mut(task) {
            Some(val) => {
                *val = true;
                Some(format!("{:#?} is now complete", task))
            }
            None => None,
        }
    }

    /// Iterates over the tasks, returning each key along with its status.
    /// If the value is true, it's considered incomplete, and if it's false, it's considered complete.
    pub fn read(&self) -> HashMap<&Task, String> {
        let mut tasks = HashMap::new();
        for (key, value) in &self.tasks {
            let status = if *value {
                "Complete".to_string()
            } else {
                "Incomplete".to_string()
            };
            tasks.insert(key, status);
        }
        tasks
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_manager_insert() {
        let mut task_manager = TaskManager::new();
        let task = Task::new("Task 1");
        assert_eq!(task_manager.insert(&task), None);
        assert_eq!(task_manager.tasks.len(), 1);
        assert_eq!(task_manager.insert(&task), Some(false));
        assert_eq!(task_manager.tasks.len(), 1);
    }

    #[test]
    fn test_task_manager_complete() {
        let mut task_manager = TaskManager::new();
        let task1 = Task::new("Task 1");
        let task2 = Task::new("Task 2");
        task_manager.insert(&task1);
        assert_eq!(task_manager.complete(&task2), None);
        assert_eq!(
            task_manager.complete(&task1),
            Some(format!("{:#?} is now complete", &task1))
        );
        assert_eq!(task_manager.tasks.get(&task1), Some(&true));
    }

    #[test]
    fn test_task_manager_save() {
        let mut task_manager = TaskManager::new();
        let task = Task::new("Task 1");
        task_manager.insert(&task);
        assert!(task_manager.save().is_ok())
    }
}
