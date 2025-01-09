mod task;

mod project;
use project::manager::ProjectManager;

fn main() {
    let mut project_manager = ProjectManager::new();

    project_manager.add_project("Learn Rust".to_string());

    match project_manager.get_project(1) {
        Some(project) => {
            println!("Project Description: {}", project.description);
            println!(
                "Created At: {}",
                project.created_at.format("%Y-%m-%d %H:%M")
            );
            project.tasks.add_task("Learn Rust".to_string());
            project.tasks.add_task("Finish Project Manager".to_string());

            println!("Project Tasks: ");
            project.tasks.list_tasks();

            project.tasks.complete_task(1);
            println!("\nTasks after completing first: ");
            project.tasks.list_tasks();
        }
        None => {
            print!("Project not Found!")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::task::manager::TaskManager;

    #[test]
    fn test_add_task() {
        let mut manager = TaskManager::new();
        manager.add_task("Prueba".to_string());

        // Usa el método público para acceder a las tareas
        assert_eq!(manager.get_tasks()[0].description, "Prueba");
    }
}
