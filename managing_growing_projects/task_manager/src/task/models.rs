// src/task/models.rs

use chrono::{DateTime, Local};

#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
    pub created_at: DateTime<Local>,
}

impl Task {
    pub fn new(id: u32, description: String) -> Self {
        Self {
            id,
            description,
            completed: false,
            created_at: Local::now(),
        }
    }
}
