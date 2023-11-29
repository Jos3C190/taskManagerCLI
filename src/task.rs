use chrono::Local;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub completed: bool,
    pub created_at: chrono::NaiveDateTime, 
    pub completed_at: Option<chrono::NaiveDateTime>,
}

impl Task {
    pub fn new(id: u32, name: String, description: String) -> Task {

        Task {
            id,
            name,
            description,
            completed: false,
            created_at: Local::now().naive_local(),
            completed_at: None,
        }
    }

    pub fn mark_as_completed(&mut self) {
        self.completed = true;
        self.completed_at = Some(Local::now().naive_local());
    }

    pub fn unmark_as_completed(&mut self) {
        self.completed = false;
        self.completed_at = None;
    }

    pub fn print_details(&self) {
        let status = if self.completed { "Completed" } else { "Pending" };
        println!("\nID Task {}", self.id);
        println!("Name: {} ({})", self.name, status);
        println!("Description: {}", self.description);
        println!("Created at: {}", self.created_at);
        if let Some(completed_at) = self.completed_at {
            println!("Completed at: {}", completed_at);
        }
    }
}
