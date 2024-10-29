use core::error;
use std::{f32::consts::E, fs, result};


fn extract_errors(text: &str) -> Vec<&str> {
    text.split('\n')
        .filter(|line| line.starts_with("ERROR"))
        .collect()
}

fn main() {

    let mut errors_log = Vec::new();

    match fs::read_to_string("logs.txt") {
        Ok(content) => {
            errors_log = extract_errors(content.as_str());
            println!("Errors: {:#?}", errors_log);
        },
        Err(error) => {
            println!("Error reading file: {}", error);
        }
    }

    println!("Errors: {:#?}", errors_log);
    
}