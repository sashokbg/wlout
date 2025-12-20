use crate::commands::common::{apply, handle_result};
use crate::model::AppData;
use wayland_client::EventQueue;

pub fn power_command(
    name: &str,
    on: bool,
    mut state: AppData,
    mut event_queue: EventQueue<AppData>,
) {
    let target_head = state.get_head(name);

    let result = apply(&mut state, &mut event_queue, |config, qh| {
        if on {
            config.enable_head(&target_head.head, &qh, ());
        } else {
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
