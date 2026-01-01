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
