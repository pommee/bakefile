mod parser;
use std::fs;

fn main() {
    let bakefile = fs::read_to_string("Bakefile").expect("Failed to read Bakefile");
    parser::parse(bakefile);
}
