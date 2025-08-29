use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Eq, PartialEq, Hash)]
struct Dep(String);

enum Action {
    Add { employee: String, department: Dep },
    RemoveDepartment(Dep),
    Get(Dep),
    GetAll,
    HelpMenu,
    Quit,
}

fn parse_input(input: &str) -> Result<Action, String> {
    if input.split_whitespace().count() > 3 {
        return Err(String::from("To many arguments"));
    }

    let mut input_vec: Vec<String> = Vec::new();
    for arg in input.split_whitespace() {
        match &arg.to_lowercase()[..] {
            "add" | "get" | "rm" => input_vec.push(arg.to_lowercase().to_owned()),
            "h" if input_vec.len() == 0 => return Ok(Action::HelpMenu),
            "q" if input_vec.len() == 0 => return Ok(Action::Quit),
            arg => {
                if input_vec.len() == 1 || input_vec.len() == 2 {
                    input_vec.push(arg.to_owned());
                }
            }
        }
    }

    if &input_vec[0][..] == "rm" {
        return Ok(Action::RemoveDepartment(Dep(input_vec[1].to_owned())));
    } else if &input_vec[0][..] == "get" {
        if input_vec.len() == 2 {
            return Ok(Action::Get(Dep(input_vec[1].to_owned())));
        } else if input_vec.len() == 1 {
            return Ok(Action::GetAll);
        } else {
            return Err(String::from("Failed to parse input"));
        }
    } else if &input_vec[0][..] == "add" && input_vec.len() == 3 {
        return Ok(Action::Add {
            employee: input_vec[1].to_owned(),
            department: Dep(input_vec[2].to_owned()),
        });
    } else {
        return Err(String::from("Failed to parse input"));
    }
}

fn print_help_menu() {
    println!("\n===HELP 'MENU'===");
    println!("\nh                       - show this info text");
    println!("\nq                       - quit");
    println!("\nadd EMPLOYEE DEPARTMENT - add an employee to an department");
    println!("                             (if the department does not exist yet,");
    println!("                              a new one will be created)");
    println!("\nget DEPARTMENT          - prints out all employees listed in the given department");
    println!("\nget                     - calls \"get DEPARTMENT\" for all available departments");
    println!("\nrm DEPARTMENT           - deletes the department");
}

fn print_dep(department: &Dep, db: &HashMap<Dep, Vec<String>>) {
    let employees = match db.get(department) {
        Some(entries) => entries,
        None => {
            println!("The department {} was not found", department.0);
            return;
        }
    };

    println!("\n===EMPLOYEES FROM \"{}\"===", department.0);
    for employee in employees {
        println!("{}", employee);
    }
}

fn execute_action(db: &mut HashMap<Dep, Vec<String>>, action: Action) -> bool {
    match action {
        Action::HelpMenu => print_help_menu(),
        Action::Add {
            employee,
            department,
        } => {
            let entry = db.entry(department).or_insert(Vec::new());
            entry.push(employee);
            entry.sort_unstable();
        }
        Action::Get(dep) => print_dep(&dep, &db),
        Action::GetAll => {
            for (dep, _) in &*db {
                print_dep(dep, &db)
            }
        }
        Action::Quit => return false,
        Action::RemoveDepartment(dep) => {
            if let Some(_) = db.remove(&dep) {
                println!("Deleted \"{}\"", dep.0);
            } else {
                println!("\"{}\" not found", dep.0);
            }
        }
    }

    return true;
}

fn main() {
    let mut db: HashMap<Dep, Vec<String>> = HashMap::new();

    println!("\n");
    'main: loop {
        print!("('h' for help)>>> ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        if let Err(_) = io::stdin().read_line(&mut input) {
            continue 'main;
        };

        let Ok(action) = parse_input(&input) else {
            println!("[ERROR]: Failed to parse input");
            continue 'main;
        };

        if !execute_action(&mut db, action) {
            /* checks if the programm should quit */
            break 'main;
        }
    }
}
