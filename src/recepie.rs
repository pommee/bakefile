pub trait RecepieUtil {
    fn new() -> Self;
    fn print(&self);
    fn reset(&mut self);
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
}
