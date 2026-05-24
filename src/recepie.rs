use std::process::Command;

pub trait RecepieUtil {
    fn new() -> Self;
    fn run(&self);
    fn reset(&mut self);
    fn has_deps(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct Recepie {
    pub name: String,
    pub dependencies: Vec<Recepie>,
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
        if self.has_deps() {
            for dep in &self.dependencies {
                dep.run();
            }
        }

        let mut full_command = String::new();
        for command in &self.commands {
            full_command.push_str(" ");
            full_command.push_str(command);
            full_command = full_command.trim().into();
        }
        Command::new("sh")
            .arg("-c")
            .arg(full_command)
            .status()
            .expect("Failed to execute command");
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
