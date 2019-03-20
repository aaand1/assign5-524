use std::io;
use std::collections::HashMap;

fn main() {
    println!("Please enter your name");
    let mut your_name: String = String::new();
    io::stdin().read_line(&mut your_name).expect("error");
    let your_name = your_name.trim();
    
    let mut members: HashMap<String, Relatives> = HashMap::new();
    members.insert(your_name.to_string(), Relatives {descendant: None, mother: None, father: None});
    
    loop {
        println!("Next action please");
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("error");
        let command: Vec<&str> = command.trim().split(" ").collect();
        
        match command[0] {
            "add" => {
                if command.len() != 4 {
                    println!("Invalid command");
                } else {
                    match members.get_mut(command[3]) {
                        Some(relatives) => {
                            match command[2] {
                                "mother" => {
                                    match &relatives.mother {
                                        Some(_) => println!("Relationship already exists"),
                                        None => relatives.mother = Some(command[1].to_string())
                                    }
                                    members.insert(command[1].to_string(), Relatives {descendant: Some(command[3].to_string()), mother: None, father: None});
                                },
                                "father" => {
                                    match &relatives.father {
                                        Some(_) => println!("Relationship already exists"),
                                        None => relatives.father = Some(command[1].to_string())
                                    }
                                    members.insert(command[1].to_string(), Relatives {descendant: Some(command[3].to_string()), mother: None, father: None});
                                },
                                _ => println!("Invalid relationship")
                            }
                        },
                        None => println!("Name not found")
                    }
                }
            },
            "delete" => {
                if command.len() != 2 {
                    println!("Invalid command");
                } else {
                    let mut orphan: String = String::new();
                    match members.get(command[1]) {
                        Some(relatives) => {
                            match &relatives.descendant {
                                Some(descendant) => {
                                    orphan = descendant.to_string();
                                    let hitlist: Vec<String> = find_ancestor(command[1].to_string(), & members);
                                    for relative in hitlist {
                                        members.remove(&relative);
                                    }
                                    println!("Delete completed");
                                },
                                None => println!("Deletion failed")
                            }
                        }
                        None => println!("Name not found")
                    }
                    match members.get_mut(&orphan) {
                        Some(relatives) => {
                            match &relatives.mother {
                                Some(mother) => {
                                    if mother == command[1] {
                                        relatives.mother = None;
                                    }
                                },
                                None => {}
                            }
                            match &relatives.father {
                                Some(father) => {
                                    if father == command[1]{
                                        relatives.father = None;
                                    }
                                },
                                None => {}
                            }
                        },
                        None => {}
                    }
                }
            },
            "print" => {
                if command.len() != 1 {
                    println!("Invalid command");
                } else {
                    print_tree(your_name.to_string(), 0, &members);
                }
            },
            "quit" => {
                if command.len() != 1{
                    println!("Invalid command");
                } else {
                    break println!("Good Bye");
                }
            }
            _ => println!("Invalid command")
        }
    }
}

struct Relatives {descendant: Option<String>, mother: Option<String>, father: Option<String>}

fn print_tree(you: String, generation: u8, members: &HashMap<String, Relatives>) {
    match members.get(&you) {
        Some(relatives) => {
            let mut index = 0;
            while index < generation {
                print!("\t");
                index = index + 1;
            }
            println!("{}", you);
            match &relatives.mother {
                Some(name) => print_tree(name.to_string(), generation+1, members),
                None => {}
            }
            match &relatives.father {
                Some(name) => print_tree(name.to_string(), generation+1, members),
                None => {}
            }
        },
        None => println!("tree broken")
    }
}

fn find_ancestor(name: String, members: &HashMap<String, Relatives>) -> Vec<String> {
    match members.get(&name) {
        Some(relatives) => {
            let mut ancestors: Vec<String> = vec![name];
            match &relatives.mother {
                Some(mother) => ancestors.extend(find_ancestor(mother.to_string(), members)),
                None => {}
            }
            match &relatives.father {
                Some(father) => ancestors.extend(find_ancestor(father.to_string(), members)),
                None => {}
            }
            return ancestors;
        },
        None => println!("Name not found")
    }
    Vec::new()
}
