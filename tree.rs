use std::io;
use std::collections::HashSet;

fn main() {
    println!("Please enter your name");
    let mut your_name: String = String::new();
    io::stdin().read_line(&mut your_name).expect("error");
    let your_name = your_name.trim();
    let mut you: Relative = Relative {name: your_name.to_string(), mother: None, father: None};
    let mut family: HashSet<String> = HashSet::new();
    family.insert(your_name.to_string());
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
                    if family.contains(&command[3].to_string()) {
                        match find(command[3].to_string(), &mut you) {
                            Some(child) => {
                                match command[2] {
                                    "mother" => {
                                        match child.mother {
                                            Some(_) => println!("Relationship already exists"),
                                            None => {
                                                if family.contains(&command[1].to_string()) {
                                                    println!("Name already exists");
                                                } else {
                                                    child.mother = Some(Box::new(Relative {name: command[1].to_string(), mother: None, father: None}));
                                                    family.insert(command[1].to_string());
                                                }
                                            }
                                        }
                                    },
                                    "father" => {
                                        match child.father {
                                            Some(_) => println!("Relationship already exists"),
                                            None => {
                                                if family.contains(&command[1].to_string()) {
                                                    println!("Name already exists");
                                                } else {
                                                    child.father = Some(Box::new(Relative {name: command[1].to_string(), mother: None, father: None}));
                                                    family.insert(command[1].to_string());
                                                }
                                            }
                                        }
                                    },
                                    _ => println!("Invalid relationship")
                                }
                            },
                            None => {}
                        }
                    } else {
                        println!("Name not found");
                    }
                }
            },
            "delete" => {
                if command.len() != 2 {
                    println!("Invalid command");
                } else {
                    if family.contains(&command[1].to_string()) {
                        if command[1].to_string() == your_name {
                            println!("Deletion failed");
                        } else {
                            let hitlist: Vec<String> = delete(command[1].to_string(), &mut you);
                            family.remove(&command[1].to_string());
                            for relative in hitlist {
                                family.remove(&relative);
                            }
                            println!("Delete complete");
                        }
                    } else {
                        println!("Name not found");
                    }
                }
            },
            "print" => {
                if command.len() != 1 {
                    println!("Invalid command");
                } else {
                    print_tree(&you, 0);
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

struct Relative {name: String, mother: Option<Box<Relative>>, father: Option<Box<Relative>>}

fn find (name: String, relative: &mut Relative) -> Option<&mut Relative> {
    if name == relative.name {
        return Some(relative)
    } else {
        match relative.mother {
            Some(ref mut mother) => {
                match find(name.to_string(), mother) {
                    None => {},
                    found => return found
                }
            },
            None => {}
        }
        match relative.father {
            Some(ref mut father) => {
                match find(name, father) {
                    None => {},
                    found => return found
                }
            },
            None => {}
        }
        return None
    }
}

fn delete (name: String, relative: &mut Relative) -> Vec<String> {
    let mut hitlist: Vec<String> = Vec::new();
    let mut removem: bool = false;
    match relative.mother {
        Some(ref mut mother) => {
            if mother.name == name {
                removem = true;
                hitlist.extend(find_ancestors(mother));
            } else {
                hitlist.extend(delete(name.to_string(), mother));
            }
        },
        None => {}
    }
    if removem {
        relative.mother = None;
    }
    let mut removef: bool = false;
    match relative.father {
        Some(ref mut father) => {
            if father.name == name {
                removef = true;
                hitlist.extend(find_ancestors(father));
            } else {
                hitlist.extend(delete(name.to_string(), father));
            }
        }
        None => {}
    }
    if removef {
        relative.father = None;
    }
    return hitlist
}

fn find_ancestors (relative: &Relative) -> Vec<String> {
    let mut ancestors: Vec<String> = vec!(relative.name.to_string());
    match relative.mother {
        Some(ref mother) => ancestors.extend(find_ancestors(mother)),
        None => {}
    }
    match relative.father {
        Some(ref father) => ancestors.extend(find_ancestors(father)),
        None => {}
    }
    return ancestors
}

fn print_tree (relative: &Relative, generation: u8) {
    for _ in 0..generation { print!("\t"); }
    println!("{}", relative.name);
    match relative.mother {
        Some(ref mother) => print_tree(mother, generation+1),
        None => {}
    }
    match relative.father {
        Some(ref father) => print_tree(father, generation+1),
        None => {}
    }
}
