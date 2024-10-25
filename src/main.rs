use std::fs;
use std::io::Error;

fn main() {

    // let text = fs::read_to_string("logs.txt").expect("Unable to read file");
    // println!("Text: {:#?}", text);

    match divide(81.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e)
    }
}

//Demonstration of error handling using division
fn divide(x: f64, y: f64) -> Result<f64, Error> {
    if y == 0.0 {
        Err(Error::other("Division by zero"))
    } else {
        Ok(x / y)
    } 
}
