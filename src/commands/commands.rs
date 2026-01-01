pub trait Executable {
    fn execute(&self);
}

pub struct InfoCommand {
    pub name: String,
}

pub struct ListCommand {
    pub verbose: bool
}