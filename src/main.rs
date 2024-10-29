use std::fs;


fn extract_errors(text: &str) -> Vec<&str> {
    text.split('\n')
        .filter(|line | line.starts_with("ERROR"))
        .map(|line| line)
        .collect()
}

fn main() {
    match fs::read_to_string("logs.txt") {
        Ok(content) => {
            let errors_log = extract_errors(content.as_str());
            
            match fs::write("errors.txt", errors_log.join("\n")) {
                Ok(_) => println!("Errors written to errors.txt"),
                Err(error) => println!("Error writing file: {}", error)
            }
        },
        Err(error) => {
            println!("Error reading file: {}", error);
        }
    }
}