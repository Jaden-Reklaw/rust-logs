use std::fs;
fn main() {

    let text = fs::read_to_string("logs.txt").expect("Unable to read file");
    println!("Text: {:#?}", text);
}
