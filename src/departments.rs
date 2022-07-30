use std::io;
use std::error;
use std::collections::HashMap;

pub fn run() -> Result<(), Box<dyn error::Error>>{
let mut org_chart: HashMap<String, Vec<String>> = HashMap::new();
    
    loop {
        prompt();

        let input = get_input().unwrap();

        let words: Vec<&str> = input.trim().split_whitespace().collect();

        let command = words[0];

        match command {
            "exit" => break,
            "add" => add_employee(&words, &mut org_chart)
                        .unwrap_or_else(handle_invalid_input),
            "get" => get_department_list(&words, &mut org_chart)
                        .unwrap_or_else(handle_invalid_input),
            _ => println!("I don't recognize that command")
        }
    }
    Ok(())
}

fn prompt(){
    println!("Enter an employee name and departmet.");
    println!("Example: Add Amir to Marketing");
    println!("->")
}

fn get_input() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

fn add_employee(words: &Vec<&str>, org: &mut HashMap<String, Vec<String>>) -> Result<(), &'static str> {
    if words.len() != 4 {
        return Err("Should be 4 words.")
    }
    
    let (name, dep) = (words[1], words[3]);
    let key = String::from(dep);

    org.entry(key)
    .and_modify(|employee_list| {
        employee_list.push(String::from(name))
    })
    .or_insert(vec![String::from(name)]);

    let department_list = org.get(dep).unwrap();

    println!("{}: {:?}", dep, department_list);
    Ok(())
}

fn get_department_list(words: &Vec<&str>, org: &mut HashMap<String, Vec<String>>) -> Result<(), &'static str> {
    if words.len() != 2 {
        return Err("Should be 2 words.")
    }
    let dep = words[1];
    match org.get(dep) {
        Some(list) => println!("Department List: {:?}", list),
        None => return Err("Could not find that department")
    }
    Ok(())
}

fn handle_invalid_input(message: &str) {
    println!("Invalid input. {}", message)
}