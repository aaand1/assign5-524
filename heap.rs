use std::io;
use std::f32;

fn main() {
    println!("Please enter your name");
    let mut your_name: String = String::new();
    io::stdin().read_line(&mut your_name).expect("error");
    let your_name = your_name.trim();
    let mut family: Vec<Option<String>> = vec!(Some(your_name.to_string()),
        None, None, None, None, None, None, None, None, None, None, None, None, None, None);
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
                    match family.iter().position(|&ref x| match x { Some(name) => name.to_string() == command[3].to_string(), None => false }) {
                        Some(child_index) => {
                            let parent_index: usize = 2 * child_index + match command[2] {
                                "mother" => 1,
                                "father" => 2,
                                _ => {
                                    println!("Invalid relationship");
                                    continue;
                                }
                            };
                            match family[parent_index] {
                                Some(_) => println!("Relationship already exists"),
                                None => if !family.contains(&Some(command[1].to_string())) {
                                            family[parent_index] = Some(command[1].to_string());
                                        } else { println!("Name already exists"); }
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
                    if command[1].to_string() == your_name {
                        println!("Deletion failed");
                    } else {
                        match family.iter().position(|&ref x| match x { Some(name) => name, None => "" } == command[3].to_string()) {
                            Some(index) => {
                                delete(&mut family, index);
                                println!("Delete completed");
                            },
                            None => println!("Name not found")
                        }
                    }
                }
            },
            "print" => {
                if command.len() != 1 {
                    println!("Invalid command");
                } else {
                    print_tree(&family, 0);
                }
            },
            "quit" => {
                if command.len() != 1 {
                    println!("Invalid command");
                } else {
                    break println!("Good Bye");
                }
            }
            _ => println!("Invalid command")
        }
    }
}

fn delete (family: &mut Vec<Option<String>>, index: usize) {
    match &family[index] {
        Some(_) => {
            family[index] = None;
            delete(family, 2*index+1);
            delete(family, 2*index+2);
        },
        None => {}
    }
}

fn print_tree (family: &Vec<Option<String>>, index: usize) {
    match &family[index] {
        Some(ref name) => {
            for _ in 0..(((index+1) as f32).log2().floor() as u8) { print!("\t"); }
            println!("{}", name);
            print_tree(family, 2*index+1);
            print_tree(family, 2*index+2);
        },
        None => {}
    }
}
