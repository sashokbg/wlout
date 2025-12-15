use crate::common::{AppData, ConfigResult};
use std::process;
use wayland_client::EventQueue;
use wayland_protocols_wlr::output_management::v1::client::zwlr_output_configuration_v1::ZwlrOutputConfigurationV1;

pub fn on_command(
    name: &str,
    mut state: AppData,
    configuration: ZwlrOutputConfigurationV1,
    mut event_queue: EventQueue<AppData>,
) {
    let target_head = state
        .heads
        .values()
        .find(|head| head.name.as_deref() == Some(name))
        .expect("requested head not found");

    let qh = event_queue.handle();

    configuration.enable_head(&target_head.head, &qh, ());
    configuration.apply();

    while state.config_result.is_none() {
        event_queue.blocking_dispatch(&mut state).unwrap();
    }

    match state.config_result.unwrap() {
        ConfigResult::Succeeded => {
            println!("Disabled {}", name)
        }
        ConfigResult::Failed => {
            eprintln!("Failed to disable {}", name);
            process::exit(1);
        }
        ConfigResult::Cancelled => {
            eprintln!("Configuration cancelled before disabling {}", name);
            process::exit(1);
        }
    }

    configuration.destroy();
}
