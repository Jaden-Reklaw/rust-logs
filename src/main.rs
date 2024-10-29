use std::fs;
use std::io::Error;


fn extract_errors(text: &str) -> Vec<&str> {
    text.split('\n')
        .filter(|line | line.starts_with("ERROR"))
        .map(|line| line)
        .collect()
}

//You can return something from main
fn main() -> Result<(), Error> {
    let text = fs::read_to_string("logs.txt")?;
    let errors_log = extract_errors(text.as_str());
    fs::write("errors.txt", errors_log.join("\n"))?;
    Ok(())
}