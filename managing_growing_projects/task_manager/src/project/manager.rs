// src/project/manager.rs
use super::models::Project;

#[derive(Debug)]
pub struct ProjectManager {
    projects: Vec<Project>,
}

impl ProjectManager {
    pub fn new() -> Self {
        Self {
            projects: Vec::new(),
        }
    }

    pub fn add_project(&mut self, description: String) {
        let id = (self.projects.len() + 1) as u32;
        let project = Project::new(id, description);
        self.projects.push(project);
    }

    pub fn get_project(&mut self, id: u32) -> Option<&mut Project> {
        for project in self
            .projects
            .iter_mut()
            .filter(|x| x.id == id)
            .collect::<Vec<&mut Project>>()
        {
            return Some(project);
        }
        return None;
    }
}
