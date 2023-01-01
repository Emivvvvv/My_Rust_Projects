pub mod structs;
use crate::structs::todo::ToDo;

fn input(info: &str) -> String {
    println!("{info}");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Something went wrong while trying to get input from the user!");
        input.pop();
        input
}

fn main() {
    let mut todo = ToDo::create_lists();

    println!("Welcome to the Emivvvvv's ToDo app!");
    println!("===================================");
    loop {
        let menu_str = "\n\n    Menu:
        1- Add item
        2- Delete item
        3- Complete item
        4- See todo list
        5- See completed items
        6- Exit";

        let user_command_input = input(menu_str);
        let command_slice = &user_command_input[..];

        match command_slice {
            "1" => todo.add_item(input("Type the new ToDo >")),
            "2" => {todo.delete_item(input("Type the ToDo will be deleted >"));},
            "3" => {todo.complete_item(input("Type the ToDo will be marked as completed >"));},
            "4" => todo.print_list(),
            "5" => todo.print_completed_list(),
            "6" => break,
            _ => println!("'{}' is not a valid input! Please type a number between 1-7 inc!", user_command_input),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_delete(){
        let mut todo = ToDo::create_lists();
        todo.add_item(String::from("add_and_delete"));
        assert_eq!(1, todo.delete_item(String::from("add_and_delete")))
    }

    #[test]
    fn add_and_complete(){
        let mut todo = ToDo::create_lists();
        todo.add_item(String::from("add_and_complete"));
        assert_eq!(1, todo.complete_item(String::from("add_and_complete")))
    }
    
    #[test]
    fn delete_nonexistent(){
        let mut todo = ToDo::create_lists();
        assert_eq!(0, todo.delete_item(String::from("delete_nonexistent")))
    }

    #[test]
    fn complete_nonexistent(){
        let mut todo = ToDo::create_lists();
        assert_eq!(0, todo.delete_item(String::from("complete_nonexistent")))
    }
}