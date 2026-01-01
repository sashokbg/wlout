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
