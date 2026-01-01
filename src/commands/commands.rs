pub trait Executable {
    fn execute(&self);
}

pub struct InfoCommand {
    pub name: String,
}

pub struct ListCommand {
    pub verbose: bool,
}

pub struct MirrorCommand {
    pub mirrored_display_name: String,
    pub reference_display_name: String,
}
