use crate::modules::city::City;

#[derive(Clone)]
pub enum Addr {
    Address(Box<Node>),
    Nil,
}

#[derive(Clone)]
pub struct Node {
    pub data: City,
    pub next: Addr,
}

pub struct LinkedList {
    pub head: Addr,
}

struct PopulationSort {
    population: i64,
    index: i64,
}

impl LinkedList {
    
    pub fn traverse(&self) {
        match &self.head {
            Addr::Address(node) => node.node_traverse(),
            Addr::Nil => println!("LinkedList is empthy!"),
        }
    }

    pub fn add(&mut self, city: City) {
        match &mut self.head {
            Addr::Address(node) => node.node_add(city),
            Addr::Nil => {
                let new_node = Node {
                    data: city,
                    next: Addr::Nil,
                };
                self.head = Addr::Address(Box::new(new_node))},
        }
    }

    pub fn delete(&mut self, user_number: i64) {
        match &mut self.head {
            Addr::Address(node) => {
                if node.data.user_number == user_number {
                    self.head = node.next.clone();
                } else {
                    node.node_delete(user_number);
                }
            }
            Addr::Nil => println!("LinkedList is empty!"),
        }
    }

    pub fn search(&self, user_number: i64) -> Option<&Node>{
        match &self.head {
            Addr::Address(node) => node.node_search(user_number),
            Addr::Nil => {println!("LinkedList is empthy!"); None},
        }
    }

    pub fn update_user_number(&mut self, old_user_number: i64, new_user_number: i64){
        match &mut self.head {
            Addr::Address(node) => node.node_update_user_number(old_user_number, new_user_number),
            Addr::Nil => println!("LinkedList is empthy!"),
        }
    }

    pub fn check_add(&mut self, city: City){
        match &mut self.head {
            Addr::Address(node) => node.node_check_add(city),
            Addr::Nil => println!("LinkedList is empthy!"),
        }
    }

    pub fn update_population(&mut self, user_number: i64, new_population: i64) {
        match &mut self.head {
            Addr::Address(node) => node.node_update_population(user_number, new_population),
            Addr::Nil => println!("LinkedList is empthy!"),
        }
    }

    pub fn print_city(&self, user_number:i64) {
        match &self.head {
            Addr::Address(node) => node.node_print_city(user_number),
            Addr::Nil => println!("LinkedList is empthy!"),
        }
    }

    pub fn delete_last(&mut self) {
        match &mut self.head {
            Addr::Address(node) => {
                match &mut node.next {
                    Addr::Nil => self.head = Addr::Nil,
                    Addr::Address(node) => node.node_delete_last(),
                }
            }
            Addr::Nil => println!("LinkedList is empthy!"),
        }
    }

    pub fn print_by_populations(&self) {
        match &self.head {
            Addr::Address(node) => node.node_print_by_populations(),
            Addr::Nil => println!("LinkedList is empthy!"),
        }
    }
}


impl Node {
    fn node_traverse(&self) {
        println!("{}",self.data);
        match &self.next {
            Addr::Address(node) => node.node_traverse(),
            Addr::Nil => return
        }
    }


    fn node_add(&mut self, city: City) {
        match &mut self.next {
            Addr::Address(node) => node.node_add(city),
            Addr::Nil => {
                let new_node = Node {
                    data: city,
                    next: Addr::Nil,
                };
                self.next = Addr::Address(Box::new(new_node));
            }
        }
    }
    fn node_delete(&mut self, user_number: i64) {
        match self.next {
            Addr::Address(ref mut node) => {
                if node.data.user_number == user_number {
                    println!("Deleting value {}", node.data.user_number);
                    self.next = node.next.clone();
                } else {
                    match node.next {
                        Addr::Nil => {
                            if node.data.user_number == user_number {
                                self.next = Addr::Nil;
                            }
                        }
                        Addr::Address(_) => node.node_delete(user_number),
                    }
                }
            }
            Addr::Nil => println!("Couldn't find the city numbered '{user_number}'"),
        }
    }

    fn node_search(&self, user_number: i64) -> Option<&Node> {
        if self.data.user_number == user_number {
            return Some(&self)
        } else {
            match &self.next {
                Addr::Address(node) => node.node_search(user_number),
                Addr::Nil => None,
            }
        }
    }
    fn node_sort_by_population(&self, mut print_order: Vec<PopulationSort>, index: i64) -> Vec<PopulationSort>{
        print_order.push(PopulationSort{population: self.data.population, index});
        match &self.next {
            Addr::Address(node) => node.node_sort_by_population(print_order, index + 1),
            Addr::Nil => return print_order,
        }
    }

    fn node_update_user_number(&mut self, old_user_number:i64, new_user_number: i64) {
        if self.data.user_number == old_user_number {
            self.data.user_number = new_user_number;
        } else {
            match &mut self.next {
                Addr::Address(node) => node.node_update_user_number(old_user_number, new_user_number),
                Addr::Nil => println!("Coulnd't find the city numbered '{old_user_number}'"),
            }
        }
    }

    fn node_update_population(&mut self, user_number:i64, new_population: i64) {
        if self.data.user_number == user_number {
            self.data.population = new_population;
        } else {
            match &mut self.next {
                Addr::Address(node) => node.node_update_population(user_number, new_population),
                Addr::Nil => println!("Coulnd't find the city numbered '{user_number}'"),
            }
        }
    }


    fn node_check_add(&mut self, city: City) {
        match self.node_search(city.user_number) {
            Some(_) => println!("This city already exists!!!!"),
            None => match &mut self.next {
                Addr::Address(node) => node.node_add(city),
                Addr::Nil => {
                    let new_node = Node {
                        data: city,
                        next: Addr::Nil,
                    };
                    self.next = Addr::Address(Box::new(new_node));
                }
            }
        }
    }

    fn node_print_city(&self, user_number: i64) {
        let to_be_printed = self.node_search(user_number);
        match to_be_printed {
            Some(node) => println!("{}", node.data),
            None => println!("Couldn't find the city numbered '{user_number}'"),
        }
    }

    fn node_delete_last(&mut self) {
        match &mut self.next {
            Addr::Address(node) => match node.next {
                Addr::Nil => self.next = Addr::Nil,
                Addr::Address(_) => node.node_delete_last(),
            }
            Addr::Nil => panic!("NODE_DELETE_LAST self.next shouldn't be Addr::Nill NEVER!!!!!"),
        }
    }

    fn node_print_by_populations(&self) {
        let mut print_order: Vec<PopulationSort> = Vec::new();
        print_order = self.node_sort_by_population(print_order, 0);
        print_order.sort_by(|a, b| b.population.cmp(&a.population));
        println!("Printing cities by their populations:");
        let mut print_index = 1;
        for city_info in print_order {
            let city_data = self.th_node_data(0, city_info.index);
            println!("{}: {} - {}", print_index, city_data.name, city_data.population);
            print_index += 1;
        }
    }

    //index won't cause error it is already checked.
    fn th_node_data(&self, curr_index: i64, target_index: i64) -> &City {
        if target_index == curr_index {
            return &self.data
        }
        match &self.next {
            Addr::Address(node) => node.th_node_data(curr_index + 1, target_index),
            Addr::Nil => panic!("Something wen't really wrong while trying to get {target_index}'th city!"),
            }
        }
    }