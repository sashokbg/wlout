use crate::backends::wlroot::connect_trait::WaylandCommand;
use crate::commands::commands::{Executable, ListCommand};
use crate::head_printer::print_heads_detail;
use crate::model::HeadInfo;

impl WaylandCommand for ListCommand {}

impl Executable for ListCommand {
    fn execute(&self) {
        let (_, state) = self.connect();

        if self.verbose {
            print_heads_detail(state.heads.into_values().collect::<Vec<HeadInfo>>())
        } else {
            for (i, head) in state.heads.iter().enumerate() {
                if i == state.heads.iter().len() - 1 {
                    print!("{}\n", head.1.name.clone().unwrap())
                } else {
                    print!("{}\t", head.1.name.clone().unwrap())
                }
            }
        }
    }
}
