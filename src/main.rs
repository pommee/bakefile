mod parser;
mod recepie;
use crate::recepie::RecepieUtil;
use std::fs;

fn main() {
    let bakefile_content = fs::read_to_string("Bakefile").expect("Failed to read Bakefile");
    let bakefile = parser::parse(bakefile_content);

    for recepie in bakefile.recepies {
        recepie.print();
    }
}
