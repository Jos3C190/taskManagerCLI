use crate::task::{self};

pub struct TaskManager {
    pub tasks: Vec<task::Task>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        TaskManager { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, name: String, description: String) {
        self.tasks.push(task::Task::new(name, description));
        println!("The task has been added.");
    }

    pub fn mark_task_completed(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            if let Some(_completed_at) = task.completed_at {
                println!("This task is already completed.");
            } else {
                task.mark_as_completed();
                println!("The task has been marked as completed.");
            }
        } else {
            println!("Invalid index. No task was marked.");
        }
    }

    pub fn delete_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks.remove(index);
            println!("The task has been eliminated.");
        } else {
            println!("Invalid index. No task was deleted.");
        }
    }

    pub fn list_tasks(&self) {
        println!("Task List: ");
        if !self.tasks.is_empty() {
            for (index, task) in self.tasks.iter().enumerate() {
                let status = if task.completed { "Completed" } else { "Pending" };
                println!("\nNo.{}: {} ({})", index, task.name, status);
                println!("Description: {}", task.description);
                println!("Created at: {}", task.created_at);
    
                if let Some(completed_at) = task.completed_at {
                    println!("Completed at: {}", completed_at);
                }
            }
        } else {
            println!("None")
        }
    }
}
