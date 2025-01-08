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
