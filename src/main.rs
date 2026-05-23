mod parser;
mod recepie;
use crate::recepie::{Recepie, RecepieUtil};
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No recepie specified, exiting. Usage: bake <recepie_name>");
        println!("Try running `bake help` for more information.");
        return;
    }

    let bakefile_content = fs::read_to_string("Bakefile").expect("Failed to read Bakefile");
    let bakefile = parser::parse(bakefile_content);

    let recepie = match bakefile.recepies.get(&args[1]) {
        Some(recepie) => recepie,
        None => {
            println!("Recepie with name '{}' was not found, existing.", args[1]);
            process::exit(1);
        }
    };

    recepie.run();
}
