use crate::backends::common::{apply, handle_result, prompt};
use crate::backends::wlroot::connect_trait::WaylandCommand;
use crate::commands::commands::{Executable, PowerCommand};
use std::process::exit;

impl WaylandCommand for PowerCommand {}

impl Executable for PowerCommand {
    fn execute(&self) {
        let (mut event_queue, mut state) = self.connect();

        let target_head = state.get_head(&self.name);
        let count = state.heads.iter().filter(|(_, head)| head.enabled).count();

        let result = apply(&mut state, &mut event_queue, |config, qh| {
            if self.on {
                config.enable_head(&target_head.head, &qh, ());
            } else {
                // off
                if count < 2 && !self.force {
                    let read =
                        prompt("You are about to power off your last display.\nProceed ? (Y/n)");

                    if read.to_lowercase() != "y" {
                        exit(1)
                    }
                }
                config.disable_head(&target_head.head);
            }
        });

        let action = if self.on { "enabled" } else { "disabled" };

        handle_result(
            result,
            &format!("Successfully {} display {}", action, self.name),
            &format!("Failed to {} display {}", action, self.name),
        )
    }
}
