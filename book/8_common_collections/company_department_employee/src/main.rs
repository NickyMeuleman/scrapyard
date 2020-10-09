use std::collections::HashMap;

enum Task {
    Add((String, String)),
    Show(String),
    ShowAll,
    Quit,
}

fn add_to_department(
    person: String,
    department: String,
    company_map: &mut HashMap<String, Vec<String>>,
) {
    let department_vec = company_map.entry(department).or_default();
    department_vec.push(person);
}

fn print_all(company_map: &HashMap<String, Vec<String>>) {
    println!("{:?}", company_map);
}

fn print_department(name: &String, company_map: &HashMap<String, Vec<String>>) {
    match company_map.get(name) {
        None => println!("No department with name: {} found.", name),
        Some(vec) => {
            let mut vec = vec.clone();
            vec.sort();
            println!("{} department: {:?}", name, vec)
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");
    input
}

fn parse_input(input: String) -> Option<Task> {
    let mut input = input.trim().split_whitespace();
    match input.next() {
        Some("add") => {
            let name = input.next();
            input.next();
            let department = input.next();
            if name != None && department != None {
                let name = name.unwrap().to_owned();
                let department = department.unwrap().to_owned();
                Some(Task::Add((name, department)))
            } else {
                println!("Please format the \"add\" command as: \"add <name> to <department>\"");
                None
            }
        }
        Some("show") => match input.next() {
            Some("all") => Some(Task::ShowAll),
            Some(name) => Some(Task::Show(name.to_owned())),
            _ => {
                println!("Please enter something after \"show\": \n 1. A department name \n 2. \"all\" to show all deparments.");
                None
            }
        },
        Some("quit") => {
            println!("Bye bye!");
            Some(Task::Quit)
        }
        _ => {
            println!("Incorrect input");
            None
        }
    }
}

fn process_task(task: Task, company_map: &mut HashMap<String, Vec<String>>) {
    match task {
        Task::Add((person, department)) => add_to_department(person, department, company_map),
        Task::Show(department) => print_department(&department, company_map),
        Task::ShowAll => print_all(company_map),
        Task::Quit => (),
    }
}

fn main() {
    let mut company_map: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Please input a command:");
        let input = get_input();
        match parse_input(input) {
            Some(Task::Quit) => break,
            Some(task) => process_task(task, &mut company_map),
            None => continue,
        }
    }
}
