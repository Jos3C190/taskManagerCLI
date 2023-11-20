use crate::task::{self, Task};
use std::{fs, io::Result};

pub struct TaskManager {
    pub tasks: Vec<task::Task>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        match TaskManager::load_from_file("data.json") {
            Ok(loaded_data) => TaskManager { tasks: loaded_data },
            Err(_) => TaskManager { tasks: Vec::new() },
        }
    }

    pub fn add_task(&mut self, name: String, description: String) {
        self.tasks.push(task::Task::new(name, description));
        self.save_tasks();
        println!("The task has been added.");
    }

    pub fn mark_task_completed(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            if task.completed_at.is_some() {
                println!("This task is already completed.");
            } else {
                task.mark_as_completed();
                self.save_tasks();
                println!("The task has been marked as completed.");
            }
        } else {
            println!("Invalid index. No task was marked.");
        }
    }

    pub fn delete_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks.remove(index);
            self.save_tasks();
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
                println!("\nTask {}", index);
                println!("Name: {} ({})", task.name, status);
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

    fn save_tasks(&self) {
        let data = serde_json::to_string(&self.tasks).unwrap();
        fs::write("data.json", data).unwrap();
    }

    fn load_from_file(file_path: &str) -> Result<Vec<Task>> {
        let data = fs::read_to_string(file_path)?;
        let tasks: Vec<Task> = serde_json::from_str(&data)?;
        Ok(tasks)
    }
}
