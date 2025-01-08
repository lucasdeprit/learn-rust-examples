mod task;
use task::manager::TaskManager;

fn main() {
    let mut manager = TaskManager::new();

    manager.add_task("Aprender Rust".to_string());
    manager.add_task("Completar el proyecto de gestor de tareas".to_string());

    println!("Lista de tareas:");
    manager.list_tasks();

    manager.complete_task(1);
    println!("\nTareas después de completar la tarea 1:");
    manager.list_tasks();
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
