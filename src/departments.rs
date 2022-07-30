use std::io;
use std::error;
use std::collections::HashMap;

pub fn prompt(){
    println!("Enter an employee name and departmet.");
    println!("Example: Add Amir to Marketing");
    println!("->")
}

pub fn run() -> Result<(), Box<dyn error::Error>>{
let mut org_chart: HashMap<String, Vec<String>> = HashMap::new();
    
    loop {
        prompt();

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim() == "exit".to_string() {
            break;
        }

        let words: Vec<&str> = input.split(' ').collect();

        if words.len() != 4 {
            println!("Invalid Input!");
            println!("Example input: Add Sam to Sales\n");
            continue;
        }

        let (name, dep) = (words[1], words[3]);

        let key = String::from(dep);

        org_chart.entry(key)
        .and_modify(|employee_list| {
            employee_list.push(String::from(name))
        })
        .or_insert(vec![String::from(name)]);

        let department_list = org_chart.get(dep).unwrap();

        println!("{}: {:?}", dep, department_list)
    }
    Ok(())
}