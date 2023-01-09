pub mod modules;

use crate::modules::linkedlist::{LinkedList, Addr};
use crate::modules::city::City;

fn string_input(info: &str) -> String {
    println!("{info}");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("ailed to read from stdin");
        input.pop();
        input
}

fn i64_input(info: &str) -> i64 {
    println!("{info}");
    let mut input_text = String::new();
    std::io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<i64>() {
        Ok(i) => i,
        Err(..) => {let new_info = "Input is not an integer! Please try again >"; i64_input(new_info)},
    }
}

fn main() {
    let mut linkedlist = LinkedList {head: Addr::Nil};

    println!("Welcome to the Emivvvvv's useless city linkedlist!");
    println!("==================================================");
    loop {
        let menu_str = "\n\nMenu:
1- Add a new city to the list
2- Add a new city to the list securily
3- Delete a city from the list with given user_number
4- Delete the last city from the list
5- Print the list
6- Update a city's population
7- Update a city's user number
8- Print the city names by their populations
9- Print informations of the given city
e- Exit";

        let user_command_input = string_input(menu_str);
        let command_slice = &user_command_input[..];

        match command_slice {
            "1" => linkedlist.add(City::create(string_input("Type the city's name >"), string_input("Type the city's state >"), i64_input("Type the city's population >"), i64_input("Type the city's user number >"))),
            "2" => linkedlist.check_add(City::create(string_input("Type the city's name >"), string_input("Type the city's state >"), i64_input("Type the city's population >"), i64_input("Type the city's user number >"))),
            "3" => linkedlist.delete(i64_input("Type the city's number that will be deleted from the list >")),
            "4" => linkedlist.delete_last(),
            "5" => linkedlist.traverse(),
            "6" => linkedlist.update_population(i64_input("Type the city's user number >"), i64_input("Type the new population >")),
            "7" => linkedlist.update_user_number(i64_input("Type the city's old user number >"), i64_input("Type the new user number >")),
            "8" => linkedlist.print_by_populations(),
            "9" => linkedlist.print_city(i64_input("Type the city's user number >")),
            "e" => break,
            _ => println!("'{}' is not a valid input! Please type a number between 1-8 inc or 'e' to exit!", user_command_input),
        }
    }
}