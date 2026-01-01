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
use crate::model::HeadModeInput;

pub struct ModeCurrentCommand {
    pub name: String,
}

pub struct ModeAutoCommand {
    pub name: String,
}

pub struct ModePreferredCommand {
    pub name: String,
}

pub struct ModeSetCommand {
    pub name: String,
    pub mode: HeadModeInput,
    pub force: bool,
}

pub struct ModeListCommand {
    pub name: String,
}

pub const REL_POS_ABOVE: &str = "above";
pub const REL_POS_BELOW: &str = "below";
pub const REL_POS_LEFT_OF: &str = "left-of";
pub const REL_POS_RIGHT_OF: &str = "right-of";

pub struct MoveRelativeCommand {
    pub moved_display_name: String,
    pub reference_display_name: String,
    pub pos: String,
}

pub struct MoveCommand {
    pub name: String,
    pub x: i32,
    pub y: i32,
}

pub struct PowerCommand {
    pub name: String,
    pub on: bool,
    pub force: bool
}