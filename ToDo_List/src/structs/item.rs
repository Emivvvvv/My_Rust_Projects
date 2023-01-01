use std::fmt;
pub struct Item {
    pub data: String,
    pub completed: bool
}


impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut _task_string = String::new();
        if self.completed {
            _task_string = String::from("is completed");
        } else {
            _task_string = String::from("is NOT completed");
        }
        write!(f, "Task is {} and it {}!", self.data, _task_string)
    }
}