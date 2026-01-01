use crate::commands::commands::{Executable};
use crate::backends::wlroot::connect_trait::WaylandCommand;
use crate::commands::commands::InfoCommand;
use crate::head_printer::print_heads_detail;

impl WaylandCommand for InfoCommand {}

impl Executable for InfoCommand {
    fn execute(&self) {
        let (_, state) = &self.connect();
        let target_head = state.get_head(self.name.as_str());

        print_heads_detail(vec![target_head])
    }
}
