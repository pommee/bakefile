use std::process::Command;

pub trait RecepieUtil {
    fn new() -> Self;
    fn run(&self);
    fn print(&self);
    fn reset(&mut self);
    fn has_deps(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct Recepie {
    pub name: String,
    pub dependencies: Vec<String>,
    pub commands: Vec<String>,
}

impl RecepieUtil for Recepie {
    fn new() -> Self {
        Recepie {
            name: String::new(),
            dependencies: Vec::new(),
            commands: Vec::new(),
        }
    }

    fn run(&self) {
        let mut full_command = String::new();
        for command in &self.commands {
            full_command.push_str(" ");
            full_command.push_str(command);
            full_command = full_command.trim().into();
        }
        println!(
            "Running recepie {} with commands:\n{}",
            self.name, full_command
        );
        Command::new("sh")
            .arg("-c")
            .arg(full_command)
            .status()
            .expect("Failed to execute command");
    }

    fn print(&self) {
        println!("{}", self.name);
        println!("  Deps: {:?}", self.dependencies);
        println!("  Commands: {:?}", self.commands);
    }

    fn reset(&mut self) {
        *self = Recepie {
            name: String::new(),
            dependencies: Vec::new(),
            commands: Vec::new(),
        };
    }

    fn has_deps(&self) -> bool {
        !self.dependencies.is_empty()
    }
}
