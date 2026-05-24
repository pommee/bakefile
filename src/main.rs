mod parser;
mod recepie;
use crate::parser::start;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No recepie specified, exiting. Usage: bake <recepie_name>");
        println!("Try running `bake help` for more information.");
        return;
    }

    let bakefile_content = fs::read_to_string("Bakefile").expect("Failed to read Bakefile");
    let bakefile = parser::parse(bakefile_content);

    start(bakefile, &args[1]);
}
