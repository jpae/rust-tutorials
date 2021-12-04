use std::io;
use std::collections::HashMap;

// TODO: Use an IO module
pub fn read_input() -> String {
    let mut selection = String::new();
    
    io::stdin()
    .read_line(&mut selection)
    .expect("Failed to read line");

    return selection
}

pub struct Company {
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    pub fn new() -> Company {
        Company {
            departments: HashMap::new(),
        }
    }

    pub fn add_employee(&mut self) {
        println!("Enter name of the Employee:");
        let name = read_input().trim().to_string();

        println!("Enter name of the Department: ");
        let dept = read_input().trim().to_string();

        println!("Adding {} to {}", name, dept);
        
        let employees = self.departments.entry(dept).or_insert(Vec::<String>::new());
        employees.push(name);
    }
    
    pub fn show_employees(&mut self) {
        println!("{:#?}", self.departments);
    }   
}