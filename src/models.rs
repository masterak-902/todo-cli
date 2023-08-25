#[derive(Debug)]
pub struct ToDo {
    task_num: Option<i32>,
    task_check: bool,
    task_event: String,
}

impl ToDo {
    // Constructor
    pub fn new(task_num: Option<i32>, task_check: bool, task_event: String) -> Self {
        Self {
            task_num,
            task_check,
            task_event,
        }
    }

    // Getter methods
    pub fn task_num(&self) -> Option<i32> {
        self.task_num
    }

    pub fn task_check(&self) -> bool {
        self.task_check
    }

    pub fn task_event(&self) -> &String {
        &self.task_event
    }
}