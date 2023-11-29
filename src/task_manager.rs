use crate::task::Task;
use std::{fs, io::Result};
use dialoguer::{theme::ColorfulTheme, Confirm};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct TaskManager {
    pub tasks: Vec<Task>,
    pub next_id: u32,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        match TaskManager::load_from_file("data.json") {
            Ok(task_manager) => task_manager,
            Err(_) => TaskManager { tasks: Vec::new(), next_id: 1 },
        }
    }

    pub fn add_task(&mut self, name: String, description: String) {
        self.tasks.push(Task::new(self.next_id, name, description));
        self.next_id += 1;
        self.save_changes();
        println!("The task has been added.");
    }

    pub fn mark_task_completed(&mut self, id: u32) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            if task.completed_at.is_some() {
                println!("This task is already completed.");
            } else {
                task.mark_as_completed();
                self.save_changes();
                println!("The task has been marked as completed.");
            }
        } else {
            println!("Invalid ID. No task was marked.");
        }
    }

    pub fn mark_all_tasks(&mut self) {
        if self.confirm_action("Do you want to mark all tasks as completed?") {
            for task in &mut self.tasks {
                if task.completed_at.is_none() {
                    task.mark_as_completed();
                }
            }
            self.save_changes();
            println!("All tasks have been marked.");
        } else {
        println!("Action cancelled");
        }
    }

    pub fn unmark_task_completed(&mut self, id: u32) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            if task.completed_at.is_none() {
                println!("This task has not yet been completed.");
            } else {
                task.unmark_as_completed();
                self.save_changes();
                println!("The task has been unmarked.");
            }
        } else {
            println!("Invalid ID. No task was unmarked.");
        }
    }

    pub fn unmark_all_tasks(&mut self) {
        if self.confirm_action("Do you want to unmark all tasks?") {
            for task in &mut self.tasks {
                if task.completed_at.is_some() {
                    task.unmark_as_completed();
                }
            }
            self.save_changes();
            println!("All tasks have been unmarked.");
        } else {
        println!("Action cancelled");
        }
    }

    pub fn delete_task(&mut self, id: u32) {
        if let Some(index) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(index);
            self.save_changes();
            println!("The task has been eliminated.");
        } else {
            println!("Invalid ID. No task was deleted.");
        }
    }

    pub fn delete_all_tasks(&mut self) {
        if self.confirm_action("Do you want to delete all tasks?") {
            self.tasks.clear();
            self.save_changes();
            println!("All tasks have been deleted.");
        } else {
            println!("Action cancelled");
        }
    }

    pub fn edit_task(&mut self, id: u32, new_name: Option<String>, new_description: Option<String>) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            if let Some(name) = new_name {
                task.name = name;
            }
            if let Some(description) = new_description {
                task.description = description;
            }

            self.save_changes();
            println!("The task has been successfully updated.");
        } else {
            println!("No task was found with ID {}", id);
        }
    }

    pub fn list_tasks(&self) {
        if !self.tasks.is_empty() {
            println!("Task List: ");
            for task in &self.tasks {
                task.print_details();
            }
        } else {
            println!("The task list is empty")
        }
    }

    fn save_changes(&self) {
        let data = serde_json::to_string(self).expect("Error serializing tasks to JSON");
        fs::write("data.json", data).expect("Error writing tasks to file");
    }

    fn load_from_file(file_path: &str) -> Result<TaskManager> {
        let data = fs::read_to_string(file_path)?;
        let task_manager: TaskManager = serde_json::from_str(&data)?;
        Ok(task_manager)
    }

    fn confirm_action(&self, prompt: &str) -> bool {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .default(false)
            .interact()
            .unwrap_or(false)
    }
}
