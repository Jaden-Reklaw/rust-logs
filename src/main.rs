use std::fs;

fn main() {

    match fs::read_to_string("logs.txt") {
        Ok(content) => {
            println!("File content: {}", content);
        },
        Err(error) => {
            println!("Error reading file: {}", error);
        }
    }
    
}