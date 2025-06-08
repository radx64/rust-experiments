
fn parse_input(input: String) -> Option<(String, String)>
{
    let mut words = input.split_whitespace();

    if words.clone().count() != 4 {
        return None
    }

    let add_word = String::from(words.clone().nth(0).unwrap_or_default());

    if add_word != "add" {
        return None
    }

    let name = String::from(words.clone().nth(1).unwrap_or_default());

    let to_word = String::from(words.clone().nth(2).unwrap_or_default());

    if to_word != "to" {
        return None
    }

    let departament = String::from(words.nth(3).unwrap_or_default());

    Some((name, departament))
}

use std::{collections::HashMap, io};

fn edit_employee(database: &mut HashMap<String, Vec<String>>) {
    loop {
        println!("Enter action in form: \"add <name> to <department>\" or \"exit\" to exit");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input: String = match input.trim().parse::<String>() {
            Ok(str) => str,
            Err(_) => continue,
        };
        if input == "exit" {
            break;
        }

        let parsed_input = match parse_input(input) {
            Some(data) => data,
            None => continue
        };

        let entry = database.entry(parsed_input.1).or_default();
        entry.push(parsed_input.0);
    }
}

fn list_employees(database: &HashMap<String, Vec<String>>) {
    let mut employees_result: Vec<String> = Vec::new();

    for (_, employees) in database {
        employees_result.append(&mut employees.clone());
    }

    employees_result.sort();

    println!("{employees_result:?}");
}

fn list_departments(database: &HashMap<String, Vec<String>>) {
    let mut department_result: Vec<String> = Vec::new();
    
    for (department, _) in database {
        department_result.push(department.clone());
    }

    department_result.sort();

    println!("{department_result:?}");
}

enum Action {
    AddEdit,
    ListEmployees,
    ListDepartments,
    Exit
}

fn get_menu_entry() -> Action {
    loop {
        println!("> ");
        println!("1 - add or edit employee");
        println!("2 - list employees");
        println!("3 - list departments");
        println!("exit -  exits");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input: String = match input.trim().parse::<String>() {
            Ok(str) => str,
            Err(_) => continue,
        };

        let result = match input.as_str() {
            "1" => Action::AddEdit,
            "2" => Action::ListEmployees,
            "3" => Action::ListDepartments,
            "exit" => Action::Exit,
            _ => {println!("Wrong option, try again."); continue}
        };
        break result;
    }
}

fn main() {

                             //Department, Name
    let mut database: HashMap<String, Vec<String>> = HashMap::new(); 

    loop {
        let action = get_menu_entry();
        match action {
            Action::Exit => break,
            Action::AddEdit => edit_employee(&mut database),
            Action::ListEmployees => list_employees(&database),
            Action::ListDepartments => list_departments(&database)
        };
    }
    println!("{database:#?}");
}
