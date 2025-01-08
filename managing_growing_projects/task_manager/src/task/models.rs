// src/task/models.rs
#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: u32, description: String) -> Self {
        Self {
            id,
            description,
            completed: false,
        }
    }
}
