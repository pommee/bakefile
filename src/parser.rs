use crate::recepie::Recepie;
use crate::recepie::RecepieUtil;

pub struct Bakefile {
    pub recepies: Vec<Recepie>,
}

pub fn parse(mut bakefile_content: String) -> Bakefile {
    bakefile_content.push_str("\n");
    println!("Parsing Bakefile of size {} bytes", bakefile_content.len());
    let mut bakefile = Bakefile {
        recepies: Vec::new(),
    };
    let mut recepie: Recepie = Recepie::new();

    bakefile_content.lines().for_each(|line| {
        if line.is_empty() && !recepie.name.is_empty() {
            println!("Empty line, finished recepie: {}", recepie.name);
            bakefile.recepies.push(recepie.clone());
            recepie.reset();
            return;
        }

        if line.is_empty() {
            println!("Empty line");
            return;
        }

        if line.starts_with("#") {
            println!("Comment: {}", line);
            return;
        }

        if line.starts_with("!") && !recepie.name.is_empty() {
            println!("Last line, finishing current recepie: {}", recepie.name);
            bakefile.recepies.push(recepie.clone());
            recepie.reset();
        }

        if line.starts_with("!") && recepie.name.is_empty() {
            let recepie_name = parse_recepie(line);
            println!("New recepie: {}", recepie_name);
            let dependencies = parse_dependencies(line);

            recepie = Recepie::new();
            recepie.name = recepie_name;
            recepie.dependencies = dependencies;
        }

        if line.starts_with("   ") {
            println!("Command line: {}", line);
            let commands = parse_commands(line);
            recepie.commands = commands;
            return;
        }
    });

    return bakefile;
}

fn parse_recepie(line: &str) -> String {
    line.split(" ")
        .nth(0)
        .unwrap_or("")
        .replace(":", "")
        .replace("!", "")
}

fn parse_dependencies(line: &str) -> Vec<String> {
    let mut dependencies: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
    dependencies.remove(0);
    return dependencies;
}

fn parse_commands(line: &str) -> Vec<String> {
    line.split_whitespace().map(|s| s.to_string()).collect()
}
