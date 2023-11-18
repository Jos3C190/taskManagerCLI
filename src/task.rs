pub struct Task {
    pub name: String,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(name: String, description: String) -> Task {

        Task {
            name,
            description,
            completed: false,
        }
    }

    pub fn mark_as_completed(&mut self) {
        self.completed = true;
    }
}
