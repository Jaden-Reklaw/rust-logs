use std::fs;


fn extract_errors(text: &str) -> Vec<String> {
    text.split('\n')
        .filter(|line | line.starts_with("ERROR"))
        .map(|s| s.to_string())
        .collect()
}

fn main() {

    let mut errors_log = Vec::new();

    match fs::read_to_string("logs.txt") {
        Ok(content) => {
            errors_log = extract_errors(content.as_str());
        },
        Err(error) => {
            println!("Error reading file: {}", error);
        }
    }

    println!("Errors: {:#?}", errors_log);
    
}