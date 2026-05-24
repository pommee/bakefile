use crate::recepie::Recepie;
use crate::recepie::RecepieUtil;
use std::process;

pub struct Bakefile {
    pub recepies: std::collections::HashMap<String, Recepie>,
}

pub fn start(bakefile: Bakefile, recepie_target: &str) {
    let recepie = match bakefile.recepies.get(recepie_target) {
        Some(recepie) => recepie,
        None => {
            println!(
                "Recepie with name {} was not found, existing.",
                recepie_target
            );
            process::exit(1);
        }
    };

    recepie.run();
}

pub fn parse(mut bakefile_content: String) -> Bakefile {
    bakefile_content.push_str("\n");
    println!("Parsing Bakefile of size {} bytes", bakefile_content.len());
    let mut bakefile = Bakefile {
        recepies: std::collections::HashMap::new(),
    };
    let mut dependency_map: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    let mut recepie: Recepie = Recepie::new();

    bakefile_content.lines().for_each(|line| {
        if line.is_empty() && !recepie.name.is_empty() {
            if bakefile.recepies.contains_key(&recepie.name) {
                recepie.reset();
                return;
            }

            bakefile
                .recepies
                .insert(recepie.name.clone(), recepie.clone());
            recepie.reset();
            return;
        }

        if line.is_empty() {
            return;
        }

        if line.starts_with("#") {
            return;
        }

        if line.starts_with("!") && !recepie.name.is_empty() {
            if bakefile.recepies.contains_key(&recepie.name) {
                recepie.reset();
                return;
            }

            bakefile
                .recepies
                .insert(recepie.name.clone(), recepie.clone());
            recepie.reset();
        }

        if line.starts_with("!") && recepie.name.is_empty() {
            let recepie_name = parse_recepie(line);
            let deps = parse_dependencies(line);
            dependency_map.insert(recepie_name.clone(), deps);

            recepie = Recepie::new();
            recepie.name = recepie_name;
        }

        if line.starts_with("   ") {
            let commands = parse_commands(line);
            recepie.commands = commands;
            return;
        }
    });

    traverse_dependencies(&mut bakefile.recepies, &dependency_map);
    return bakefile;
}

fn resolve_deps(
    name: &str,
    dep_map: &std::collections::HashMap<String, Vec<String>>,
    acc: &mut Vec<String>,
    seen: &mut std::collections::HashSet<String>,
) {
    if let Some(deps) = dep_map.get(name) {
        for dep in deps {
            if seen.contains(dep) {
                continue;
            }
            seen.insert(dep.clone());
            resolve_deps(dep, dep_map, acc, seen);
            acc.push(dep.clone());
        }
    }
}

fn traverse_dependencies(
    recepies: &mut std::collections::HashMap<String, Recepie>,
    dep_map: &std::collections::HashMap<String, Vec<String>>,
) {
    for recepie_name in dep_map.keys() {
        let mut resolved_names: Vec<String> = Vec::new();
        let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
        resolve_deps(recepie_name, dep_map, &mut resolved_names, &mut seen);

        let mut dep_clones: Vec<Recepie> = Vec::new();
        for dep_name in &resolved_names {
            if let Some(d) = recepies.get(dep_name) {
                dep_clones.push(d.clone());
            } else {
                println!(
                    "Warning: dependency '{}' for '{}' not found",
                    dep_name, recepie_name
                );
            }
        }

        if let Some(recepie) = recepies.get_mut(recepie_name) {
            for d in dep_clones {
                recepie.dependencies.push(d);
            }
        }
    }
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
    if !dependencies.is_empty() {
        dependencies.remove(0);
    }
    return dependencies;
}

fn parse_commands(line: &str) -> Vec<String> {
    line.split_whitespace().map(|s| s.to_string()).collect()
}
