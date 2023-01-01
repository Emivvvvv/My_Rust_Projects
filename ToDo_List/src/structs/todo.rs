use crate::structs::item::Item;

pub struct ToDo {
    pub uncompleted_list: Vec<Item>,
    pub completed_list: Vec<Item>
}
    
    
impl ToDo {
    pub fn create_lists() -> ToDo {
        let todo_list = ToDo {
            uncompleted_list: Vec::<Item>::new(),
            completed_list: Vec::<Item>::new(),
        };
        todo_list
    }
        
    pub fn add_item(&mut self, user_data_input: String) {
        let new_item = Item {
            data: user_data_input,
            completed: false
        };
        self.uncompleted_list.push(new_item);
    }
        
    pub fn delete_item(&mut self, to_delete: String) -> i32 {
        match self.uncompleted_list.iter().position(|x| *x.data == to_delete) {
            Some(i) => {self.uncompleted_list.remove(i);1},
            None => {println!("Couldn't find {to_delete} todo! Please try another todo!");0},
        } 
    }
        
    pub fn complete_item(&mut self, to_make_completed: String) -> i32{
        match self.uncompleted_list.iter().position(|x| *x.data == to_make_completed) {
            Some(i) => {
                let mut completed_item = self.uncompleted_list.remove(i);
                completed_item.completed = true;
                self.completed_list.push(completed_item);
                1},
            None => {println!("Couldn't find {to_make_completed} todo! Please try another todo!");0},
        }   
    }
        
    pub fn print_list(&self) {
        let mut index = 0;
        for item in &self.uncompleted_list {
            println!("{} - {}", index + 1, item);
            index += 1;
        }
        if index == 0 {
            println!("You don't have any ToDos! You can add one with typing '1' command.");
        }
    }
        
    pub fn print_completed_list(&self) {
        let mut index = 0;
        for item in &self.completed_list {
            println!("{} - {}", index + 1, item);
            index += 1;
        }
        if index == 0 {
            println!("You don't have any completed ToDos!");
        }
    }
}