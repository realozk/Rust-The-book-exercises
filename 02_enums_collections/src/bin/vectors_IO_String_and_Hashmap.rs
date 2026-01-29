use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    println!("CLI for company");
    println!("Commands: 'Add', 'List ', 'All', 'Quit' \n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() { continue; }

        match parts[0] {
            "Add" => {
                if parts.len() < 4 {
                    println!("Usage: Add <Name> to <Dept>");
                } else {
                    let name = parts[1].to_string();
                    let dept = parts[3].to_string();
                    company.entry(dept).or_default().push(name);
                    println!("Added.");
                }
            },
            "List" => {
                if parts.len() < 2 {
                    println!("Usage: List <Dept>");
                } else {
                    let dept = parts[1];
                    match company.get(dept) {
                        Some(employees) => {
                            let mut sorted_emps = employees.clone();
                            sorted_emps.sort();
                            for emp in sorted_emps {
                                println!("- {}", emp);
                            }
                        },
                        None => println!("Department not found."),
                    }
                }
            },
            "All" => {
                let mut departments: Vec<_> = company.keys().collect();
                departments.sort();

                for dept in departments {
                    println!("\n[{}]", dept);
                    let mut employees = company[dept].clone();
                    employees.sort();
                    for emp in employees {
                        println!("- {}", emp);
                    }
                }
            },
            "Quit" => break,
            _ => println!("Unknown command."),
        }
    }
}