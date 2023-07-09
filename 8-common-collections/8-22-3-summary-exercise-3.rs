use std::collections::HashMap;
use std::io;

fn main() {
    println!("\nUsing a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company; for example,\n\n\"Add Sally to Engineering\" or \"Add Amir to Sales\"\n\nThen let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.\n");

    #[derive(Debug)]
    struct Company {
        departments: HashMap<String, Vec<String>>,
    }

    impl Company {
        fn add_employee(&mut self, name: String, department: String) {
            let company_department = self.departments.entry(String::from(&department)).or_insert(vec![]);
            company_department.push(String::from(&name));
            company_department.sort();
            println!("Added employee {name} to {department} department\n");
        }

        fn list_employees_from(&self, department: &str) {
            let company_department = self.departments.get(department);
            match company_department {
                Some(dep) => println!("{department} department employees:\n{:#?}\n", dep),
                None => println!("Invalid \"{department}\" department\n"),
            }
        }

        fn list_employees(&self) {
            println!("{:#?}\n", self);
        }
    }

    let mut company = Company {
        departments: HashMap::from([
            (String::from("Engineering"), vec![]),
            (String::from("Sales"), vec![]),
        ]),
    };
    
    loop {
        let mut user_input = String::new();
        if let Err(_) = io::stdin().read_line(&mut user_input) {}
        let mut split_user_input = user_input.split_whitespace().clone();
        let command = match split_user_input.nth(0) {
            Some(command) => command,
            None => continue
        };
        if command.to_lowercase().eq("add") { 
            let mut name = String::new();
            let mut department = String::new();
            match split_user_input.nth(0) {
                Some(employee_name) => name.push_str(employee_name),
                None => {
                    println!("Invalid add operation: unspecified employee name\n");
                    continue;
                }
            };
            match split_user_input.nth(1) {
                Some(employee_department) => department.push_str(employee_department),
                None => {
                    println!("Invalid add operation: unspecified employee department\n");
                    continue;
                }
            };
            company.add_employee(name, department);
            continue;
        }
        if command.to_lowercase().eq("list") {
            match split_user_input.nth(0) {
                Some(list_department) => company.list_employees_from(list_department),
                None => company.list_employees()
            }
            continue;
        }
        if command.to_lowercase().eq("quit") { break; }
        println!("Invalid \"{command}\" command");
    }
}

