// src/task/manager.rs
use super::models::Task;

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, description: String) {
        let id = (self.tasks.len() + 1) as u32;
        let task = Task::new(id, description);
        self.tasks.push(task);
    }

    pub fn list_tasks(&self) {
        for task in &self.tasks {
            println!(
                "[{}] {} - {} - Created At: {}",
                task.id,
                if task.completed { "X" } else { " " },
                task.description,
                task.created_at.format("%Y-%m-%d %H:%M"),
            );
        }
    }

    pub fn complete_task(&mut self, id: u32) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
        }
    }
}
