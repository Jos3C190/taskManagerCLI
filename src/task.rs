use chrono::Local;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub description: String,
    pub completed: bool,
    pub created_at: chrono::NaiveDateTime, 
    pub completed_at: Option<chrono::NaiveDateTime>,
}

impl Task {
    pub fn new(name: String, description: String) -> Task {
        let created_at = Local::now().naive_local();

        Task {
            name,
            description,
            completed: false,
            created_at,
            completed_at: None,
        }
    }

    pub fn mark_as_completed(&mut self) {
        self.completed = true;
        self.completed_at = Some(Local::now().naive_local());
    }
}
