use crate::commands::common::{apply, handle_result, prompt};
use crate::model::AppData;
use std::process::exit;
use wayland_client::EventQueue;

pub fn power_command(
    name: &str,
    on: bool,
    mut state: AppData,
    mut event_queue: EventQueue<AppData>,
    force: &bool,
) {
    let target_head = state.get_head(name);
    let count = state.heads.iter().filter(|(_, head)| head.enabled).count();

    let result = apply(&mut state, &mut event_queue, |config, qh| {
        if on {
            config.enable_head(&target_head.head, &qh, ());
        } else {
            // off
            if count < 2 && !force {
                let read = prompt("You are about to power off your last display.\nProceed ? (Y/n)");

                if read.to_lowercase() != "y" {
                    exit(1)
                }
            }
            config.disable_head(&target_head.head);
        }
    });

    let action = if on { "enabled" } else { "disabled" };

    handle_result(
        result,
        &format!("Successfully {} display {}", action, name),
        &format!("Failed to {} display {}", action, name),
    )
}
