use chrono::{DateTime, Local};

use crate::task::manager::TaskManager;

#[derive(Debug)]
pub struct Project {
    pub id: u32,
    pub description: String,
    pub created_at: DateTime<Local>,
    pub tasks: TaskManager,
}

impl Project {
    pub fn new(id: u32, description: String) -> Self {
        Self {
            id,
            description,
            created_at: Local::now(),
            tasks: TaskManager::new(),
        }
    }
}
